use std::borrow::Cow;

use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;

use fluvio::consumer::Record;
use fluvio::Offset;
use fluvio_connector_common::{tracing::info, LocalBoxSink, Sink};
use url::Url;

use redis::JsonAsyncCommands;

use crate::config::RedisConfig;

#[derive(Debug, Clone)]
pub(crate) struct RedisSink {
    pub(crate) prefix: String,
    pub(crate) url: Url,
    pub(crate) to_hash: Option<bool>,
}

impl RedisSink {
    pub(crate) fn new(config: &RedisConfig) -> Result<Self> {
        let prefix = config.prefix.clone();
        let url = Url::parse(&config.url.resolve()?).context("unable to parse Redis url")?;
        let to_hash = config.to_hash;

        Ok(Self {
            prefix,
            url,
            to_hash,
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
            async move {
                let value = String::from_utf8_lossy(record.value());
                if let Some(true) = self.to_hash {
                    info!("Using set");
                    redis::cmd("SET")
                        .arg(&[key, value.to_string()])
                        .query_async(&mut con)
                        .await?;
                } else {
                    con.json_set(key, "$".to_string(), &value).await?;
                }

                Ok::<_, anyhow::Error>(con)
            }
        });
        Ok(Box::pin(unfold))
    }
}
