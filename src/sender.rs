use crate::store::db::Store;
use carapax::{
    core::{
        methods::{EditMessageText, SendMessage},
        types::{Message, ParseMode},
        Api,
    },
    HandlerFuture, HandlerResult,
};
use futures::{future::Either, Future};

#[derive(Clone)]
pub struct MessageSender {
    api: Api,
    db_store: Store,
}

impl MessageSender {
    pub fn new(api: Api, db_store: Store) -> Self {
        Self { api, db_store }
    }

    /// Send a new or edit already sent message with given text
    ///
    /// # Arguments
    ///
    /// * incoming_message - Message from update to track to
    /// * text - Text to send
    pub fn send(
        &self,
        incoming_message: &Message,
        text: String,
        reply_to: ReplyTo,
    ) -> HandlerFuture {
        let db_store = self.db_store.clone();
        let api = self.api.clone();
        let chat_id = incoming_message.get_chat_id();
        let incoming_message_id = incoming_message.id;
        let reply_to_id = match reply_to {
            ReplyTo::Incoming => incoming_message_id,
            ReplyTo::Reply => match incoming_message.reply_to {
                Some(ref reply_to) => reply_to.id,
                None => incoming_message_id,
            },
        };

        macro_rules! send_new {
            () => {
                api.execute(
                    SendMessage::new(chat_id, text)
                        .reply_to_message_id(reply_to_id)
                        .parse_mode(ParseMode::Html),
                )
                .and_then(move |result_message| {
                    db_store
                        .track_message(incoming_message_id, result_message.id)
                        .map(|()| HandlerResult::Continue)
                })
            };
        }

        if incoming_message.is_edited() {
            HandlerFuture::new(db_store.get_tracked_message(incoming_message_id).and_then(
                move |tracked_message_id| {
                    if let Some(tracked_message_id) = tracked_message_id {
                        Either::A(
                            api.execute(
                                EditMessageText::new(chat_id, tracked_message_id, text)
                                    .parse_mode(ParseMode::Html),
                            )
                            .map(|_| HandlerResult::Continue),
                        )
                    } else {
                        Either::B(send_new!())
                    }
                },
            ))
        } else {
            HandlerFuture::new(send_new!())
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ReplyTo {
    /// Reply to incoming message
    Incoming,
    /// Reply to `reply_to` message if exists
    Reply,
}
