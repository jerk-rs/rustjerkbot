use crate::context::Context;
use carapax::{
    handler,
    methods::SendMessage,
    types::{Message, MessageData, ParseMode},
    ExecuteError,
};
use std::{error::Error, fmt};
use tokio_postgres::Error as PostgresError;

#[handler]
pub async fn handle_new_chat_member(context: &Context, input: Message) -> Result<(), NewChatMemberError> {
    if let MessageData::NewChatMembers(_) = input.data {
        let rows = context
            .pg_client
            .query("SELECT text FROM greetings ORDER BY RANDOM() LIMIT 1", &[])
            .await
            .map_err(NewChatMemberError::GetGreeting)?;
        if rows.is_empty() {
            return Ok(());
        }
        let greeting: String = rows[0].get(0);
        context
            .api
            .execute(
                SendMessage::new(context.config.chat_id, greeting)
                    .reply_to_message_id(input.id)
                    .parse_mode(ParseMode::Html),
            )
            .await
            .map_err(NewChatMemberError::SendMessage)?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum NewChatMemberError {
    GetGreeting(PostgresError),
    SendMessage(ExecuteError),
}

impl Error for NewChatMemberError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NewChatMemberError::GetGreeting(err) => Some(err),
            NewChatMemberError::SendMessage(err) => Some(err),
        }
    }
}

impl fmt::Display for NewChatMemberError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NewChatMemberError::GetGreeting(err) => write!(out, "failed to get greeting: {}", err),
            NewChatMemberError::SendMessage(err) => write!(out, "failed to send message: {}", err),
        }
    }
}
