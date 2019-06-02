use carapax::{
    context::Context,
    core::{
        methods::SendMessage,
        types::{Message, ParseMode},
        Api,
    },
    HandlerFuture, HandlerResult,
};
use futures::Future;

const FERRIS: &str = r#"
          \
           \
              _~^~^~_
          \) /  o o  \ (/
            '_   -   _'
            / '-----' \
"#;

const WIDTH: usize = 15;

fn say(input: &str, width: usize) -> String {
    let mut result = String::new();
    let bar_buffer: String = std::iter::repeat('-').take(width + 4).collect();
    result += &bar_buffer;
    result.push('\n');
    for i in input.split(|x: char| x == '\n') {
        for j in i.chars().collect::<Vec<char>>().as_slice().chunks(width) {
            result += "| ";
            result.extend(j);
            for _ in 0..width - j.len() {
                result.push(' ');
            }
            result += " |\n";
        }
    }
    result += &bar_buffer;
    result += FERRIS;
    result
}

pub fn handle_ferris(context: &mut Context, message: Message, args: Vec<String>) -> HandlerFuture {
    let maybe_text = args.join(" ");
    let maybe_text = maybe_text.trim();
    let input_text = if maybe_text.is_empty() {
        String::from("You should provide some text")
    } else {
        String::from(maybe_text)
    };
    let api = context.get::<Api>().clone();
    let result = say(&input_text, WIDTH);
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
