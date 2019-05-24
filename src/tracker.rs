use crate::{config::Config, store::Store};
use carapax::{
    context::Context,
    core::{methods::GetChatMember, types::Update, Api},
    HandlerFuture, HandlerResult,
};
use futures::Future;

pub fn handle_update(context: &mut Context, update: Update) -> HandlerFuture {
    let user_id = update.get_user().map(|x| x.id);
    let chat_id = update.get_chat_id();
    if let (Some(user_id), Some(chat_id)) = (user_id, chat_id) {
        let config = context.get::<Config>();
        if config.chat_id == chat_id {
            let api = context.get::<Api>();
            let store = context.get::<Store>().clone();
            return HandlerFuture::new(api.execute(GetChatMember::new(chat_id, user_id)).and_then(
                move |member| {
                    store
                        .set_user(member.user().clone())
                        .map(|()| HandlerResult::Continue)
                },
            ));
        }
    }
    HandlerResult::Continue.into()
}
