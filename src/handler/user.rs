use crate::sender::MessageSender;
use carapax::{context::Context, core::types::Message, HandlerFuture};

pub fn get_user_info(context: &mut Context, message: Message, _args: Vec<String>) -> HandlerFuture {
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
    context.get::<MessageSender>().send(&message, data)
}
