use crate::{sender::MessageSender, store::autoresponse::MessageStore};
use carapax::{
    context::Context,
    core::types::{Message, MessageData},
    Handler, HandlerFuture, HandlerResult,
};
use std::sync::Arc;

pub struct AutoresponseHandler {
    msg_store: Arc<MessageStore>,
}

impl AutoresponseHandler {
    pub fn new(msg_store: MessageStore) -> Self {
        Self {
            msg_store: Arc::new(msg_store),
        }
    }
}

impl Handler for AutoresponseHandler {
    type Input = Message;
    type Output = HandlerFuture;

    fn handle(&self, context: &mut Context, message: Self::Input) -> Self::Output {
        let msg_store = self.msg_store.clone();
        let message_sender = context.get::<MessageSender>();
        message
            .get_text()
            .and_then(|text| msg_store.find_for_text(&text.data))
            .map(|reply| reply.message)
            .or_else(|| {
                if let MessageData::NewChatMembers(ref users) = message.data {
                    users.iter().next().and_then(|user| {
                        msg_store
                            .find_for_new_member_user(user)
                            .or_else(|| msg_store.find_for_new_member())
                    })
                } else {
                    None
                }
            })
            .map(|text| message_sender.send(&message, text))
            .unwrap_or_else(|| HandlerResult::Continue.into())
    }
}
