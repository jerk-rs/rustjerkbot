use carapax::{
    context::Context,
    core::{
        methods::SendMessage,
        types::{Message, ParseMode},
        Api,
    },
    CommandHandler, HandlerFuture, HandlerResult,
};
use futures::{
    future::{self, Either},
    Future,
};

pub mod transform;

use self::transform::TransformResult;

pub struct TransformCommand<T> {
    transform: T,
    monospace_reply: bool,
}

impl<T> TransformCommand<T> {
    pub fn new(transform: T) -> Self {
        Self {
            transform,
            monospace_reply: true,
        }
    }

    pub fn without_monospace_reply(mut self) -> Self {
        self.monospace_reply = false;
        self
    }
}

impl<T> CommandHandler for TransformCommand<T>
where
    T: Fn(&str) -> TransformResult<String>,
{
    type Output = HandlerFuture;

    fn handle(&self, context: &mut Context, message: Message, args: Vec<String>) -> Self::Output {
        let chat_id = message.get_chat_id();
        let message_id = match message.reply_to {
            Some(ref reply_to) => reply_to.id,
            None => message.id,
        };
        let maybe_text = args.join(" ");
        let maybe_text = maybe_text.trim();
        let maybe_text = if maybe_text.is_empty() {
            message
                .reply_to
                .and_then(|x| x.get_text().map(|x| x.data.clone()))
        } else {
            Some(String::from(maybe_text))
        };
        let monospace = self.monospace_reply;
        let api = context.get::<Api>().clone();
        HandlerFuture::new(if let Some(text) = maybe_text {
            Either::A(
                future::result((self.transform)(&text))
                    .from_err()
                    .and_then(move |text| {
                        let text = if monospace {
                            format!("```\n{}\n```", text)
                        } else {
                            text
                        };
                        api.execute(
                            SendMessage::new(chat_id, text)
                                .reply_to_message_id(message_id)
                                .parse_mode(ParseMode::Markdown),
                        )
                        .map(|_| HandlerResult::Continue)
                    }),
            )
        } else {
            Either::B(
                api.execute(
                    SendMessage::new(chat_id, "You should provide some text")
                        .reply_to_message_id(message_id),
                )
                .map(|_| HandlerResult::Continue),
            )
        })
    }
}
