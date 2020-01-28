use crate::{
    context::Context,
    sender::{ReplyTo, SendError},
};
use carapax::{
    handler,
    types::{Message, ParseMode},
};
use sedregex::find_and_replace;

#[handler]
pub async fn replace_text_handler(context: &Context, message: Message) -> Result<(), SendError> {
    let source = match message.reply_to {
        Some(ref reply_to) => reply_to.get_text(),
        None => None,
    };
    if let (Some(commands), Some(text)) = (message.get_text(), source) {
        let commands = commands
            .data
            .split('\n')
            .filter_map(complete_command)
            .collect::<Vec<String>>();
        if commands.is_empty() {
            return Ok(());
        }
        let reply_text = match find_and_replace(&text.data, commands) {
            Ok(reply_text) => reply_text.to_string(),
            Err(err) => err.to_string(),
        };
        context
            .message_sender
            .send(
                &message,
                if reply_text.is_empty() {
                    String::from("Result text can not be empty")
                } else if reply_text.len() > 4096 {
                    String::from("Result text can not exceed 4096 characters")
                } else {
                    ParseMode::Html.escape(&reply_text)
                },
                ReplyTo::Reply,
            )
            .await?;
    }
    Ok(())
}

fn complete_command(line: &str) -> Option<String> {
    if !(line.starts_with("s/") || line.starts_with("/s/")) {
        return None;
    }
    let out = if line.starts_with('/') { &line[1..] } else { &line };
    let n_slashes = out.matches('/').count();
    let n_escaped_slashes = out.matches("\\/").count();
    let mut out = String::from(out);
    if (n_slashes - n_escaped_slashes) == 2 {
        out.push('/');
    }
    Some(out)
}
