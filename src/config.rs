use carapax::core::{types::Integer, Config as ApiConfig};
use failure::Error;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
struct RawConfig {
    token: String,
    proxy: Option<String>,
    webhook_address: Option<String>,
    #[serde(default = "default_webhook_path")]
    webhook_path: String,
    redis_url: String,
    chat_id: Integer,
    shippering_pair_timeout: u64,
    shippering_message_timeout: u64,
}

fn default_webhook_path() -> String {
    String::from("/")
}

#[derive(Clone, Debug)]
pub struct Config {
    token: String,
    proxy: Option<String>,
    pub webhook_url: Option<(SocketAddr, String)>,
    pub redis_url: String,
    pub chat_id: Integer,
    pub shippering_pair_timeout: u64,
    pub shippering_message_timeout: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        let raw: RawConfig = envy::prefixed("RUSTJERKBOT_").from_env()?;
        let webhook_url = match raw.webhook_address {
            Some(addr) => Some((addr.parse::<SocketAddr>()?, raw.webhook_path)),
            None => None,
        };
        Ok(Config {
            token: raw.token,
            proxy: raw.proxy,
            webhook_url,
            redis_url: raw.redis_url,
            chat_id: raw.chat_id,
            shippering_pair_timeout: raw.shippering_pair_timeout,
            shippering_message_timeout: raw.shippering_message_timeout,
        })
    }

    pub fn get_api_config(&self) -> ApiConfig {
        let mut config = ApiConfig::new(self.token.clone());
        if let Some(ref proxy) = self.proxy {
            config = config.proxy(proxy.clone());
        }
        config
    }
}
