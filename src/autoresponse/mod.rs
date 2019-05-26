use carapax::{
    context::Context,
    core::{
        methods::SendMessage,
        types::{Message, MessageData, ParseMode},
        Api,
    },
    Handler, HandlerFuture, HandlerResult,
};
use futures::{
    future::{self, Either},
    Future,
};
use std::sync::Arc;

mod message;

pub use self::message::MessageStore;

pub struct AutoresponseHandler {
    store: Arc<MessageStore>,
}

impl AutoresponseHandler {
    pub fn new(store: MessageStore) -> Self {
        Self {
            store: Arc::new(store),
        }
    }
}

impl Handler for AutoresponseHandler {
    type Input = Message;
    type Output = HandlerFuture;

    fn handle(&self, context: &mut Context, message: Self::Input) -> Self::Output {
        let chat_id = message.get_chat_id();
        let api = context.get::<Api>().clone();
        let store = self.store.clone();
        let reply_to_message_id = match message.reply_to {
            Some(ref x) => x.id,
            None => message.id,
        };
        let reply = message
            .get_text()
            .and_then(|text| store.find_for_text(&text.data))
            .map(|reply| {
                (
                    if reply.reply_to {
                        reply_to_message_id
                    } else {
                        message.id
                    },
                    reply.message,
                )
            })
            .or_else(|| {
                if let MessageData::NewChatMembers(ref users) = message.data {
                    users
                        .iter()
                        .next()
                        .and_then(|user| {
                            store
                                .find_for_new_member_user(user)
                                .or_else(|| store.find_for_new_member())
                        })
                        .map(|text| (message.id, text))
                } else {
                    None
                }
            });
        HandlerFuture::new(if let Some((message_id, text)) = reply {
            Either::A(
                api.execute(
                    SendMessage::new(chat_id, text)
                        .reply_to_message_id(message_id)
                        .parse_mode(ParseMode::Markdown),
                )
                .map(|_| HandlerResult::Continue),
            )
        } else {
            Either::B(future::ok(HandlerResult::Continue))
        })
    }
}
