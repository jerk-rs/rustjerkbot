use crate::{
    context::Context,
    sender::{ReplyTo, SendError},
};
use carapax::{handler, Command};

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

#[handler(command = "/fsays")]
pub async fn handle_ferris(context: &Context, command: Command) -> Result<(), SendError> {
    let maybe_text = command.get_args().join(" ");
    let maybe_text = maybe_text.trim();
    macro_rules! empty_text {
        () => {
            String::from("You should provide some text")
        };
    }
    let message = command.get_message();
    let input_text = if maybe_text.is_empty() {
        match message.reply_to {
            Some(ref reply_to) => reply_to
                .get_text()
                .map(|x| x.data.clone())
                .unwrap_or_else(|| empty_text!()),
            None => empty_text!(),
        }
    } else {
        String::from(maybe_text)
    };
    let data = format!("<pre>{}\n</pre>", say(&input_text, WIDTH));
    context.message_sender.send(&message, data, ReplyTo::Reply).await
}
