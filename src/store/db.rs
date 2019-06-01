use crate::entities::{Pair, UserData};
use carapax::core::types::{Integer, User};
use failure::Error;
use futures::{
    future::{self, Either},
    Future,
};
use redis::{r#async::SharedConnection, Client, Cmd, FromRedisValue};

const NAMESPACE: &str = "rustjerkbot";
const TRACK_MESSAGE_TIMEOUT: u32 = 172_800;

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
                    .map(|conn| Self { conn })
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
                        pair.mark_as_old();
                        Ok(Some(pair))
                    } else {
                        Ok(None)
                    }
                }
                None => Ok(None),
            })
    }

    pub fn track_message(
        &self,
        input_message_id: Integer,
        result_message_id: Integer,
    ) -> impl Future<Item = (), Error = Error> {
        let mut cmd = redis::cmd("SETEX");
        let key = format!("{}:{}", format_key("track_messages"), input_message_id);
        cmd.arg(key);
        cmd.arg(TRACK_MESSAGE_TIMEOUT);
        cmd.arg(result_message_id);
        self.query(cmd)
    }

    pub fn get_tracked_message(
        &self,
        input_message_id: Integer,
    ) -> impl Future<Item = Option<Integer>, Error = Error> {
        let mut cmd = redis::cmd("GET");
        let key = format!("{}:{}", format_key("track_messages"), input_message_id);
        cmd.arg(key);
        self.query(cmd)
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
