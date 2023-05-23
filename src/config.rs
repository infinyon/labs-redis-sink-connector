use fluvio_connector_common::{connector, secret::SecretString};

#[connector(config, name = "redis")]
#[derive(Debug)]
pub(crate) struct RedisConfig {
    pub prefix: String,
    pub url: SecretString,
    pub to_hash: Option<bool>,
}
