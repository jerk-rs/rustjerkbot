use carapax::core::{types::Integer, Config as ApiConfig};
use failure::{Error, Fail};
use std::{
    env::{var, VarError},
    ffi::OsString,
    net::SocketAddr,
    num::ParseIntError,
    str::FromStr,
};

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
        let webhook_url = match get_var_string_opt("RUSTJERKBOT_WEBHOOK_ADDRESS")? {
            Some(addr) => {
                let addr = addr.parse::<SocketAddr>()?;
                let path = get_var_string_opt("RUSTJERKBOT_WEBHOOK_PATH")?
                    .unwrap_or_else(|| String::from("/"));
                Some((addr, path))
            }
            None => None,
        };

        Ok(Config {
            token: get_var_string("RUSTJERKBOT_TOKEN")?,
            proxy: get_var_string_opt("RUSTJERKBOT_PROXY")?,
            webhook_url,
            redis_url: get_var_string("RUSTJERKBOT_REDIS_URL")?,
            chat_id: get_var_number("RUSTJERKBOT_CHAT_ID")?,
            shippering_pair_timeout: get_var_number("RUSTJERKBOT_SHIPPERING_PAIR_TIMEOUT")?,
            shippering_message_timeout: get_var_number("RUSTJERKBOT_SHIPPERING_MESSAGE_TIMEOUT")?,
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

#[derive(Debug, Fail)]
enum ConfigError {
    #[fail(display = "'{}' does not contain an unicode string: {:?}", _0, _1)]
    BadData(String, OsString),
    #[fail(display = "'{}' is not specified", _0)]
    Missing(String),
    #[fail(display = "'{}' does not contain a number: {}", _0, _1)]
    NotNumber(String, ParseIntError),
}

fn get_var_string<S>(name: S) -> Result<String, ConfigError>
where
    S: Into<String>,
{
    let name = name.into();
    var(&name).map_err(|e| match e {
        VarError::NotPresent => ConfigError::Missing(name),
        VarError::NotUnicode(data) => ConfigError::BadData(name, data),
    })
}

fn get_var_string_opt<S>(name: S) -> Result<Option<String>, ConfigError>
where
    S: Into<String>,
{
    let name = name.into();
    match var(&name) {
        Ok(value) => Ok(Some(value)),
        Err(VarError::NotPresent) => Ok(None),
        Err(VarError::NotUnicode(data)) => Err(ConfigError::BadData(name, data)),
    }
}

fn get_var_number<S, N>(name: S) -> Result<N, ConfigError>
where
    S: Into<String>,
    N: FromStr<Err = ParseIntError>,
{
    let name = name.into();
    get_var_string(name.clone()).and_then(|value| {
        value
            .parse::<N>()
            .map_err(|e| ConfigError::NotNumber(name, e))
    })
}
