
use std::borrow::Cow;
use std::collections::HashMap;

use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;

use fluvio::consumer::Record;
use fluvio::Offset;

use fluvio_connector_common::{tracing::info, LocalBoxSink, Sink};
use url::Url;

use redis::JsonAsyncCommands;

use crate::config::RedisConfig;

type KVRecord = HashMap<String, String>;
#[derive(Debug, Clone)]
pub(crate) struct RedisSink {
    pub(crate) prefix: String,
    pub(crate) url: Url,
    pub(crate) operation: Option<String>,
}

impl RedisSink {
    pub(crate) fn new(config: &RedisConfig) -> Result<Self> {
        let prefix = config.prefix.clone();
        let url = Url::parse(&config.url.resolve()?).context("unable to parse Redis url")?;
        let operation = config.operation.clone();

        Ok(Self {
            prefix,
            url,
            operation,
        })
    }
}

#[async_trait]
impl Sink<Record> for RedisSink {
    async fn connect(self, _offset: Option<Offset>) -> Result<LocalBoxSink<Record>> {
        info!("Connecting to Redis");
        let client = redis::Client::open(self.url)?;
        let con = client.get_async_connection().await?;
        info!("Connected to Redis");
        info!("Prefix: {}", &self.prefix);
        let unfold = futures::sink::unfold(con, move |mut con, record: Record| {
            let key = if let Some(key) = record.key() {
                String::from_utf8_lossy(key)
            } else {
                Cow::Owned(record.timestamp().to_string())
            };
            let key = format!("{}:{}", self.prefix, key);
            println!("key: {}", key);
            let operation = if self.operation.is_some(){
                let operation=self.operation.clone().unwrap();
                let operation = operation.to_uppercase();
                operation
            }else{
                String::from("JSON")
            };
            let prefix = self.prefix.clone();
            async move {
                match operation.as_str() {
                    "SET" => {

                    }
                    "TS.ADD" => {
                        info!("Using TS.ADD");
                        let mut kvs: KVRecord= serde_json::from_slice(record.value())?;
                        info!("Using Operation: {}", &operation);
                        println!("kvs: Key {:?}", kvs["key"]);
                        println!("kvs: Value {:?}", kvs["value"]);
                        println!("{}",prefix);
                        let full_key=format!("{}_{}", &prefix, kvs.remove("key").unwrap());
                        println!("Full key {}",full_key);
                        let timestamp = if kvs.contains_key("timestamp"){
                            let timestamp=kvs.remove("timestamp").unwrap();
                            if timestamp.len()==13{
                                let timestamp=timestamp.to_string();
                                timestamp
                            }else{
                                "*".to_string()
                            }
                        } else{
                            "*".to_string()
                        };
                        redis::cmd("TS.ADD")
                        .arg(&[full_key, timestamp, kvs["value"].to_string()])
                        .query_async(&mut con)
                        .await?;
                    }
                    _ => {
                        info!("Using JSON.SET");
                        let value = String::from_utf8_lossy(record.value());
                        con.json_set(key, "$".to_string(), &value).await?;
                    }
                }

                Ok::<_, anyhow::Error>(con)
            }
        });
        Ok(Box::pin(unfold))
    }
}
