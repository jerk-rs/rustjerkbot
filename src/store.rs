use carapax::core::types::{Integer, User};
use failure::Error;
use futures::{
    future::{self, Either},
    Future,
};
use redis::{r#async::SharedConnection, Client, Cmd, FromRedisValue};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

const NAMESPACE: &str = "rustjerkbot";

#[derive(Clone)]
pub struct Store {
    conn: SharedConnection,
}

impl Store {
    pub fn open<R: AsRef<str>>(redis_url: R) -> impl Future<Item = Self, Error = Error> {
        future::result(Client::open(redis_url.as_ref()))
            .from_err()
            .and_then(|client| {
                client
                    .get_shared_async_connection()
                    .from_err()
                    .map(|conn| Store { conn })
            })
    }

    pub fn get_user_ids(&self) -> impl Future<Item = Vec<Integer>, Error = Error> {
        let mut cmd = redis::cmd("HKEYS");
        cmd.arg(format_key("users"));
        self.query(cmd)
    }

    pub fn get_user(
        &self,
        user_id: Integer,
    ) -> impl Future<Item = Option<UserData>, Error = Error> {
        let mut cmd = redis::cmd("HGET");
        cmd.arg(format_key("users"));
        cmd.arg(user_id);
        self.query::<Option<String>>(cmd).and_then(|result| {
            if let Some(data) = result {
                match serde_json::from_str::<UserData>(&data) {
                    Ok(data) => Ok(Some(data)),
                    Err(err) => Err(err.into()),
                }
            } else {
                Ok(None)
            }
        })
    }

    pub fn set_user(&self, user: User) -> impl Future<Item = (), Error = Error> {
        let user_id = user.id;
        let data = UserData::from(user);
        match serde_json::to_string(&data) {
            Ok(data) => {
                let mut cmd = redis::cmd("HSET");
                cmd.arg(format_key("users"));
                cmd.arg(user_id);
                cmd.arg(data);
                Either::A(self.query(cmd))
            }
            Err(err) => Either::B(future::err(err.into())),
        }
    }

    pub fn del_users(&self, user_ids: &[Integer]) -> impl Future<Item = (), Error = Error> {
        let mut cmd = redis::cmd("HDEL");
        cmd.arg(format_key("users"));
        for user_id in user_ids {
            cmd.arg(*user_id);
        }
        self.query(cmd)
    }

    pub fn save_pair(&self, pair: Pair) -> impl Future<Item = Pair, Error = Error> {
        match serde_json::to_string(&pair) {
            Ok(serialized_pair) => {
                let mut cmd = redis::cmd("LPUSH");
                cmd.arg(format_key("pairs"));
                cmd.arg(serialized_pair);
                Either::A(self.query(cmd).map(|()| pair))
            }
            Err(err) => Either::B(future::err(err.into())),
        }
    }

    /// Returns a last pair for given timeout in seconds
    ///
    /// A pair created within timeout (secs) will be returned, or none otherwise
    pub fn find_last_pair(&self, timeout: u64) -> impl Future<Item = Option<Pair>, Error = Error> {
        let mut cmd = redis::cmd("LINDEX");
        cmd.arg(format_key("pairs"));
        cmd.arg(0);
        self.query(cmd)
            .and_then(move |pair: Option<String>| match pair {
                Some(pair) => {
                    let mut pair: Pair = serde_json::from_str(&pair)?;
                    if pair.is_alive(timeout)? {
                        pair.is_old = true;
                        Ok(Some(pair))
                    } else {
                        Ok(None)
                    }
                }
                None => Ok(None),
            })
    }

    fn query<V>(&self, cmd: Cmd) -> impl Future<Item = V, Error = Error>
    where
        V: FromRedisValue + Send + 'static,
    {
        cmd.query_async(self.conn.clone())
            .from_err()
            .map(|(_conn, v)| v)
    }
}

fn format_key(key: &str) -> String {
    format!("{}:{}", NAMESPACE, key)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pair {
    active_user: UserData,
    passive_user: UserData,
    timestamp: u64,
    #[serde(skip_serializing, skip_deserializing)]
    is_old: bool,
}

impl Pair {
    pub fn new(active_user: UserData, passive_user: UserData) -> Result<Self, Error> {
        Ok(Self {
            active_user,
            passive_user,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|x| x.as_secs())?,
            is_old: false,
        })
    }

    pub fn active_user(&self) -> &UserData {
        &self.active_user
    }

    pub fn passive_user(&self) -> &UserData {
        &self.passive_user
    }

    fn is_alive(&self, timeout: u64) -> Result<bool, Error> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|x| x.as_secs())?;
        Ok(self.timestamp >= now - timeout)
    }

    pub fn is_old(&self) -> bool {
        self.is_old
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserData {
    id: Integer,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
}

impl UserData {
    pub fn mention(&self) -> String {
        if let Some(ref username) = self.username {
            format!("@{}", username)
        } else {
            let mut full_name = self.first_name.clone();
            if let Some(ref last_name) = self.last_name {
                full_name += last_name;
            }
            full_name = ammonia::clean(&full_name);
            format!(r#"<a href="tg://user?id={}">{}</a>"#, self.id, full_name)
        }
    }

    pub fn custom_mention(&self, name: &str) -> String {
        let name = ammonia::clean(name);
        format!(r#"<a href="tg://user?id={}">{}</a>"#, self.id, name)
    }
}

impl From<User> for UserData {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
        }
    }
}
