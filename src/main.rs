mod config;
mod sink;
use config::RedisConfig;

use sink::RedisSink;

use fluvio_connector_common::Sink;
use fluvio_connector_common::{connector, consumer::ConsumerStream, Result};
use futures::SinkExt;

#[connector(sink)]
async fn start(config: RedisConfig, mut stream: impl ConsumerStream) -> Result<()> {
    println!("Starting redis-connector-sink sink connector with {config:?}");
    let sink = RedisSink::new(&config)?;
    let mut sink = sink.connect(None).await?;
    while let Some(item) = stream.next().await {
        let record = item?;
        sink.send(record).await?;
    }
    Ok(())
}
