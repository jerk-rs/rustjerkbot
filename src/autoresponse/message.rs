use carapax::core::types::{User, UserId};
use failure::Error;
use rand::{seq::SliceRandom, thread_rng};
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::Read, path::Path};

#[derive(Debug, Deserialize)]
struct RawMessageStore {
    new_member: HashMap<String, Vec<String>>,
    text: HashMap<String, Vec<RawMessages>>,
}

#[derive(Debug, Deserialize)]
struct RawMessages {
    input: String,
    output: Vec<String>,
    reply_to: Option<bool>,
}

#[derive(Debug, Default)]
pub struct MessageStore {
    new_member: Vec<String>,
    new_member_user: Vec<(UserId, Vec<String>)>,
    contains: HashMap<String, Messages>,
    equals: HashMap<String, Messages>,
    matches: Vec<(Regex, Messages)>,
}

#[derive(Debug)]
struct Messages {
    reply_to: bool,
    output: Vec<String>,
}

pub struct Reply {
    pub message: String,
    pub reply_to: bool,
}

impl Messages {
    fn choose(&self) -> Option<Reply> {
        self.output.choose(&mut thread_rng()).map(|message| Reply {
            message: message.clone(),
            reply_to: self.reply_to,
        })
    }
}

impl MessageStore {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut f = File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        let mut raw_store: RawMessageStore = serde_yaml::from_slice(&buf)?;
        let mut store = Self::default();
        if let Some(all) = raw_store.new_member.remove("all") {
            store.new_member = all;
        }
        for (key, val) in raw_store.new_member {
            let user_id = match key.parse::<i64>() {
                Ok(val) => UserId::from(val),
                Err(_) => UserId::from(key),
            };
            store.new_member_user.push((user_id, val));
        }
        if let Some(raw_messages) = raw_store.text.remove("contains") {
            for raw_message in raw_messages {
                store.contains.insert(
                    raw_message.input,
                    Messages {
                        reply_to: raw_message.reply_to.unwrap_or(true),
                        output: raw_message.output,
                    },
                );
            }
        }
        if let Some(raw_messages) = raw_store.text.remove("equals") {
            for raw_message in raw_messages {
                store.equals.insert(
                    raw_message.input,
                    Messages {
                        reply_to: raw_message.reply_to.unwrap_or(true),
                        output: raw_message.output,
                    },
                );
            }
        }
        if let Some(raw_messages) = raw_store.text.remove("matches") {
            for raw_message in raw_messages {
                store.matches.push((
                    Regex::new(&raw_message.input)?,
                    Messages {
                        reply_to: raw_message.reply_to.unwrap_or(true),
                        output: raw_message.output,
                    },
                ));
            }
        }
        Ok(store)
    }

    pub fn find_for_new_member(&self) -> Option<String> {
        self.new_member.choose(&mut thread_rng()).cloned()
    }

    pub fn find_for_new_member_user(&self, user: &User) -> Option<String> {
        self.new_member_user
            .iter()
            .find(|x| compare_user_id(&x.0, user))
            .and_then(|x| x.1.choose(&mut thread_rng()).cloned())
    }

    pub fn find_for_text(&self, text: &str) -> Option<Reply> {
        self.equals
            .get(text)
            .and_then(|x| x.choose())
            .or_else(|| {
                self.contains
                    .iter()
                    .find(|(key, _)| text.contains(key.as_str()))
                    .and_then(|(_, x)| x.choose())
            })
            .or_else(|| {
                self.matches
                    .iter()
                    .find(|(key, _)| key.is_match(text))
                    .and_then(|(_, x)| x.choose())
            })
    }
}

fn compare_user_id(a: &UserId, b: &User) -> bool {
    if let (UserId::Username(a), Some(b)) = (a, &b.username) {
        a == b
    } else if let UserId::Id(a) = a {
        a == &b.id
    } else {
        false
    }
}
