use crate::context::Context;
use carapax::{
    methods::{GetChatMember, SendMessage},
    types::{ChatMember, Integer, User},
    ExecuteError,
};
use liquid::{value::liquid_value, Error as LiquidError};
use std::{error::Error, fmt};
use tokio::time::delay_for;
use tokio_postgres::Error as PostgresError;

pub struct Shippering {
    context: Context,
}

impl Shippering {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    async fn get_chat_member(&self, user_id: Integer) -> Result<ChatMember, ShipperingError> {
        Ok(self
            .context
            .api
            .execute(GetChatMember::new(self.context.config.chat_id, user_id))
            .await
            .map_err(ShipperingError::GetChatMember)?)
    }

    async fn delete_user(&self, user_id: Integer) -> Result<(), ShipperingError> {
        self.context
            .pg_client
            .execute("DELETE from users WHERE id = $1", &[&user_id])
            .await
            .map_err(ShipperingError::DeleteUser)?;
        Ok(())
    }

    async fn find_pair(&self) -> Result<Option<Pair>, ShipperingError> {
        loop {
            let rows = self
                .context
                .pg_client
                .query("select id from users order by random() limit 2", &[])
                .await
                .map_err(ShipperingError::GetUsers)?;
            if rows.len() != 2 {
                return Ok(None);
            }
            let first_id = rows[0].get(0);
            let first_member = self.get_chat_member(first_id).await?;
            if !first_member.is_member() {
                self.delete_user(first_id).await?;
                continue;
            }
            let last_id = rows[1].get(0);
            let last_member = self.get_chat_member(last_id).await?;
            if !last_member.is_member() {
                self.delete_user(last_id).await?;
                continue;
            }
            return Ok(Some(Pair {
                first: first_member,
                last: last_member,
            }));
        }
    }

    async fn send_message(&self, pair: &Pair) -> Result<(), ShipperingError> {
        let rows = self
            .context
            .pg_client
            .query("SELECT template FROM shippering_phrases ORDER BY random() limit 1", &[])
            .await
            .map_err(ShipperingError::GetPhrase)?;
        if rows.is_empty() {
            return Ok(());
        }
        let template: String = rows[0].get(0);
        let template = self.context.tpl_parser.parse(&template)?;
        let first = mention(pair.first.get_user());
        let last = mention(pair.last.get_user());
        let vars = liquid_value!({
            "first": first,
            "last": last
        })
        .into_object()
        .expect("Can not convert template vars into object");
        let rendered = template.render(&vars)?.trim().to_string();
        for line in rendered.split("\\n") {
            self.context
                .api
                .execute(SendMessage::new(self.context.config.chat_id, line))
                .await
                .map_err(ShipperingError::SendMessage)?;
            delay_for(self.context.config.shippering_message_timeout).await
        }
        Ok(())
    }

    pub async fn run(self) -> Result<(), ShipperingError> {
        loop {
            if let Some(pair) = self.find_pair().await? {
                self.send_message(&pair).await?;
            }
            delay_for(self.context.config.shippering_pair_timeout).await;
        }
    }
}

struct Pair {
    first: ChatMember,
    last: ChatMember,
}

fn mention(user: &User) -> String {
    if let Some(ref username) = user.username {
        format!("@{}", username)
    } else {
        let mut full_name = user.first_name.clone();
        if let Some(ref last_name) = user.last_name {
            full_name += last_name;
        }
        full_name = ammonia::clean(&full_name);
        format!(r#"<a href="tg://user?id={}">{}</a>"#, user.id, full_name)
    }
}

#[derive(Debug)]
pub enum ShipperingError {
    GetChatMember(ExecuteError),
    GetUsers(PostgresError),
    DeleteUser(PostgresError),
    GetPhrase(PostgresError),
    Liquid(LiquidError),
    SendMessage(ExecuteError),
}

impl From<LiquidError> for ShipperingError {
    fn from(err: LiquidError) -> Self {
        ShipperingError::Liquid(err)
    }
}

impl Error for ShipperingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ShipperingError::GetChatMember(err) => Some(err),
            ShipperingError::GetUsers(err) => Some(err),
            ShipperingError::DeleteUser(err) => Some(err),
            ShipperingError::GetPhrase(err) => Some(err),
            ShipperingError::Liquid(err) => Some(err),
            ShipperingError::SendMessage(err) => Some(err),
        }
    }
}

impl fmt::Display for ShipperingError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShipperingError::GetChatMember(err) => write!(out, "failed to get chat member: {}", err),
            ShipperingError::GetUsers(err) => write!(out, "failed to get users: {}", err),
            ShipperingError::DeleteUser(err) => write!(out, "failed to delete user: {}", err),
            ShipperingError::GetPhrase(err) => write!(out, "failed to get shippering phrase: {}", err),
            ShipperingError::Liquid(err) => write!(out, "template error: {}", err),
            ShipperingError::SendMessage(err) => write!(out, "failed to send message: {}", err),
        }
    }
}
