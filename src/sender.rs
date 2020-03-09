use carapax::{
    methods::{EditMessageText, SendMessage},
    session::{backend::redis::RedisBackend as RedisSessionBackend, SessionError, SessionIdError, SessionManager},
    types::{Message, ParseMode},
    Api, ExecuteError,
};
use std::{error::Error, fmt};

const TRACK_MESSAGE_PREFIX: &str = "message_sender:";
const TRACK_MESSAGE_TIMEOUT: u64 = 172_800;

#[derive(Clone)]
pub struct MessageSender {
    api: Api,
    session_manager: SessionManager<RedisSessionBackend>,
}

impl MessageSender {
    pub fn new(api: Api, session_manager: SessionManager<RedisSessionBackend>) -> Self {
        Self { api, session_manager }
    }

    async fn send_new(&self, incoming_message: &Message, text: String, reply_to: ReplyTo) -> Result<(), SendError> {
        let chat_id = incoming_message.get_chat_id();
        let reply_to_id = match reply_to {
            ReplyTo::Incoming => incoming_message.id,
            ReplyTo::Reply => match incoming_message.reply_to {
                Some(ref reply_to) => reply_to.id,
                None => incoming_message.id,
            },
        };
        let result_message = self
            .api
            .execute(
                SendMessage::new(chat_id, text)
                    .reply_to_message_id(reply_to_id)
                    .parse_mode(ParseMode::Html),
            )
            .await?;

        let mut session = self.session_manager.get_session(incoming_message)?;
        let key = format!("{}{}", TRACK_MESSAGE_PREFIX, incoming_message.id);
        session.set(&key, &result_message.id).await?;
        session.expire(key, TRACK_MESSAGE_TIMEOUT).await?;

        Ok(())
    }

    /// Send a new or edit already sent message with given text
    ///
    /// # Arguments
    ///
    /// * incoming_message - Message from update to track to
    /// * text - Text to send
    pub async fn send(&self, incoming_message: &Message, text: String, reply_to: ReplyTo) -> Result<(), SendError> {
        let chat_id = incoming_message.get_chat_id();
        if incoming_message.is_edited() {
            let mut session = self.session_manager.get_session(incoming_message)?;
            let key = format!("{}{}", TRACK_MESSAGE_PREFIX, incoming_message.id);
            let tracked_message_id = session.get(key).await?;
            if let Some(tracked_message_id) = tracked_message_id {
                self.api
                    .execute(EditMessageText::new(chat_id, tracked_message_id, text).parse_mode(ParseMode::Html))
                    .await?;
                return Ok(());
            }
        }
        self.send_new(&incoming_message, text, reply_to).await?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ReplyTo {
    /// Reply to incoming message
    Incoming,
    /// Reply to `reply_to` message if exists
    Reply,
}

#[derive(Debug)]
pub enum SendError {
    ExecuteMethod(ExecuteError),
    Session(SessionError),
    SessionId(SessionIdError),
}

impl From<ExecuteError> for SendError {
    fn from(err: ExecuteError) -> Self {
        SendError::ExecuteMethod(err)
    }
}

impl From<SessionError> for SendError {
    fn from(err: SessionError) -> Self {
        SendError::Session(err)
    }
}

impl From<SessionIdError> for SendError {
    fn from(err: SessionIdError) -> Self {
        SendError::SessionId(err)
    }
}

impl Error for SendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SendError::ExecuteMethod(err) => Some(err),
            SendError::Session(err) => Some(err),
            SendError::SessionId(err) => Some(err),
        }
    }
}

impl fmt::Display for SendError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let reason = match self {
            SendError::ExecuteMethod(err) => format!("execute method error: {}", err),
            SendError::Session(err) => format!("session error: {}", err),
            SendError::SessionId(err) => format!("{}", err),
        };
        write!(out, "can not send message: {}", reason)
    }
}
