use crate::{
    context::Context,
    sender::{ReplyTo, SendError},
};
use carapax::{handler, types::Command};

#[handler(command = "/user")]
pub async fn get_user_info(context: &Context, command: Command) -> Result<(), SendError> {
    let message = command.get_message();
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
    context.message_sender.send(&message, data, ReplyTo::Incoming).await
}
