use carapax::{
    context::Context,
    core::{
        methods::SendMessage,
        types::{Message, ParseMode},
        Api,
    },
    HandlerFuture, HandlerResult,
};
use ferris_says::say;
use futures::Future;

const WIDTH: usize = 24;

pub fn handle_ferris(context: &mut Context, message: Message, args: Vec<String>) -> HandlerFuture {
    let maybe_text = args.join(" ");
    let maybe_text = maybe_text.trim();
    let input_text = if maybe_text.is_empty() {
        String::from("You should provide some text")
    } else {
        String::from(maybe_text)
    };
    let api = context.get::<Api>().clone();
    let mut output_text = Vec::<u8>::new();
    let result = match say(input_text.as_bytes(), WIDTH, &mut output_text) {
        Ok(()) => match String::from_utf8(output_text) {
            Ok(text) => text,
            Err(e) => e.to_string(),
        },
        Err(e) => e.to_string(),
    };
    let result = format!("```\n{}\n```", result);
    let chat_id = message.get_chat_id();
    let message_id = match message.reply_to {
        Some(ref reply_to) => reply_to.id,
        None => message.id,
    };
    HandlerFuture::new(
        api.execute(
            SendMessage::new(chat_id, result)
                .parse_mode(ParseMode::Markdown)
                .reply_to_message_id(message_id),
        )
        .map(|_| HandlerResult::Continue),
    )
}
