use carapax::{
    context::Context,
    core::{methods::SendMessage, types::Message, Api},
    HandlerFuture, HandlerResult,
};
use futures::Future;

pub fn handle_user(context: &mut Context, message: Message, _args: Vec<String>) -> HandlerFuture {
    let user = match message.reply_to {
        Some(ref reply_to) => reply_to.get_user(),
        None => message.get_user(),
    };
    let data = match user {
        Some(user) => format!(
            "ID: {}\nFIRST NAME: {}\nLAST NAME: {:?}\nUSERNAME: {:?}\nLANGUAGE CODE: {:?}",
            user.id, user.first_name, user.last_name, user.username, user.language_code
        ),
        None => String::from("No user found in a message"),
    };
    let api = context.get::<Api>().clone();
    HandlerFuture::new(
        api.execute(SendMessage::new(message.get_chat_id(), data).reply_to_message_id(message.id))
            .map(|_| HandlerResult::Continue),
    )
}
