use self::{
    arrow::Arrow, base::TransformText, cw::Cw, huify::Huify, reverse::Reverse, square::Square,
    star::Star,
};
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

mod arrow;
mod base;
mod cw;
mod huify;
mod reverse;
mod square;
mod star;

pub struct TransformCommand<T> {
    transformer: T,
    monospace_reply: bool,
}

impl TransformCommand<Arrow> {
    pub fn arrow() -> Self {
        Self {
            transformer: Arrow::new(),
            monospace_reply: true,
        }
    }
}

impl TransformCommand<Cw> {
    pub fn cw() -> Self {
        Self {
            transformer: Cw::new(),
            monospace_reply: true,
        }
    }
}

impl TransformCommand<Huify> {
    pub fn huify() -> Self {
        Self {
            transformer: Huify::new(),
            monospace_reply: false,
        }
    }
}

impl TransformCommand<Reverse> {
    pub fn reverse() -> Self {
        Self {
            transformer: Reverse,
            monospace_reply: false,
        }
    }
}

impl TransformCommand<Square> {
    pub fn square() -> Self {
        Self {
            transformer: Square::new(),
            monospace_reply: true,
        }
    }
}

impl TransformCommand<Star> {
    pub fn star() -> Self {
        Self {
            transformer: Star::new(),
            monospace_reply: true,
        }
    }
}

impl<T> CommandHandler for TransformCommand<T>
where
    T: TransformText,
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
                future::result(self.transformer.transform(&text))
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
