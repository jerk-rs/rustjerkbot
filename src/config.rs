use carapax::{types::Integer, Config as ApiConfig, ParseProxyError};
use envy::Error as EnvyError;
use serde::Deserialize;
use std::{
    error::Error,
    fmt,
    net::{AddrParseError, SocketAddr},
};

#[derive(Debug, Deserialize)]
struct RawConfig {
    token: String,
    proxy: Option<String>,
    webhook_address: Option<String>,
    #[serde(default = "default_webhook_path")]
    webhook_path: String,
    postgres_url: String,
    redis_url: String,
    chat_id: Integer,
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
    pub postgres_url: String,
    pub chat_id: Integer,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let raw: RawConfig = envy::prefixed("RUSTJERKBOT_").from_env()?;
        let webhook_url = match raw.webhook_address {
            Some(addr) => Some((
                addr.parse::<SocketAddr>().map_err(ConfigError::WebhookAddress)?,
                raw.webhook_path,
            )),
            None => None,
        };
        Ok(Config {
            token: raw.token,
            proxy: raw.proxy,
            webhook_url,
            redis_url: raw.redis_url,
            postgres_url: raw.postgres_url,
            chat_id: raw.chat_id,
        })
    }

    pub fn get_api_config(&self) -> Result<ApiConfig, ConfigError> {
        let mut config = ApiConfig::new(self.token.clone());
        if let Some(ref proxy) = self.proxy {
            config = config.proxy(proxy.clone())?;
        }
        Ok(config)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Envy(EnvyError),
    ProxyAddress(ParseProxyError),
    WebhookAddress(AddrParseError),
}

impl From<EnvyError> for ConfigError {
    fn from(err: EnvyError) -> Self {
        ConfigError::Envy(err)
    }
}

impl From<ParseProxyError> for ConfigError {
    fn from(err: ParseProxyError) -> Self {
        ConfigError::ProxyAddress(err)
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::Envy(err) => Some(err),
            ConfigError::ProxyAddress(err) => Some(err),
            ConfigError::WebhookAddress(err) => Some(err),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Envy(err) => write!(out, "{}", err),
            ConfigError::ProxyAddress(err) => write!(out, "bad proxy address: {}", err),
            ConfigError::WebhookAddress(err) => write!(out, "bad webhook address: {}", err),
        }
    }
}
