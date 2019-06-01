use carapax::core::types::{Integer, User};
use failure::Error;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

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

    /// Returns true if pair created within given timeout and false otherwise
    pub fn is_alive(&self, timeout: u64) -> Result<bool, Error> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|x| x.as_secs())?;
        Ok(self.timestamp >= now - timeout)
    }

    pub fn is_old(&self) -> bool {
        self.is_old
    }

    pub fn mark_as_old(&mut self) {
        self.is_old = true;
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
