use crate::{
    context::Context,
    sender::{ReplyTo, SendError},
};
use carapax::{async_trait, types::Message, Handler};
use rand::{seq::SliceRandom, thread_rng};
use regex::{Error as RegexError, Regex};
use std::{collections::HashMap, error::Error, fmt, sync::Arc};
use tokio_postgres::{Client as PgClient, Error as PostgresError};

pub struct AutoresponseHandler {
    contains: HashMap<String, Messages>,
    equals: HashMap<String, Messages>,
    matches: Vec<(Regex, Messages)>,
}

impl AutoresponseHandler {
    pub async fn new(pg_client: Arc<PgClient>) -> Result<Self, AutoresponseError> {
        let mut contains = HashMap::new();
        let mut equals = HashMap::new();
        let mut matches = Vec::new();

        for row in pg_client
            .query(
                "SELECT input, rule_type, reply_to, output FROM autoresponse_phrases",
                &[],
            )
            .await
            .map_err(AutoresponseError::GetPhrases)?
        {
            let input: String = row.get(0);
            let rule_type: String = row.get(1);
            let reply_to: bool = row.get(2);
            let mut output: Vec<String> = row.get(3);
            output = output.into_iter().map(|x| x.replace("\\n", "\n")).collect();
            let messages = Messages { reply_to, output };
            match rule_type.as_str() {
                "contains" => {
                    contains.insert(input, messages);
                }
                "equals" => {
                    equals.insert(input, messages);
                }
                "matches" => matches.push((Regex::new(&input)?, messages)),
                _ => return Err(AutoresponseError::UnknownRuleType(rule_type)),
            }
        }

        Ok(Self {
            contains,
            equals,
            matches,
        })
    }

    fn find_for_text(&self, text: &str) -> Option<Reply> {
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

#[async_trait]
impl Handler<Context> for AutoresponseHandler {
    type Input = Message;
    type Output = Result<(), AutoresponseError>;

    async fn handle(&mut self, context: &Context, message: Self::Input) -> Self::Output {
        if let Some(text) = message.get_text() {
            if let Some(reply) = self.find_for_text(&text.data) {
                context
                    .message_sender
                    .send(
                        &message,
                        reply.message,
                        if reply.reply_to {
                            ReplyTo::Reply
                        } else {
                            ReplyTo::Incoming
                        },
                    )
                    .await?
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Messages {
    reply_to: bool,
    output: Vec<String>,
}

impl Messages {
    fn choose(&self) -> Option<Reply> {
        self.output.choose(&mut thread_rng()).map(|message| Reply {
            message: message.clone(),
            reply_to: self.reply_to,
        })
    }
}

struct Reply {
    message: String,
    reply_to: bool,
}

#[derive(Debug)]
pub enum AutoresponseError {
    GetPhrases(PostgresError),
    Regex(RegexError),
    Send(SendError),
    UnknownRuleType(String),
}

impl From<RegexError> for AutoresponseError {
    fn from(err: RegexError) -> Self {
        AutoresponseError::Regex(err)
    }
}

impl From<SendError> for AutoresponseError {
    fn from(err: SendError) -> Self {
        AutoresponseError::Send(err)
    }
}

impl Error for AutoresponseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AutoresponseError::GetPhrases(err) => Some(err),
            AutoresponseError::Regex(err) => Some(err),
            AutoresponseError::Send(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for AutoresponseError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AutoresponseError::GetPhrases(err) => write!(out, "failed to get phrases: {}", err),
            AutoresponseError::Regex(err) => write!(out, "failed to parse regex: {}", err),
            AutoresponseError::Send(err) => write!(out, "failed to send message: {}", err),
            AutoresponseError::UnknownRuleType(rule_type) => write!(out, "unknown rule type: {}", rule_type),
        }
    }
}
