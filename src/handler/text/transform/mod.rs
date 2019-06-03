use self::{
    arrow::Arrow, base::TransformText, cw::Cw, huify::Huify, reverse::Reverse, square::Square,
    star::Star,
};
use crate::sender::{MessageSender, ReplyTo};
use carapax::{context::Context, core::types::Message, CommandHandler, HandlerFuture};

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
        let maybe_text = args.join(" ");
        let maybe_text = maybe_text.trim();
        let maybe_text = if maybe_text.is_empty() {
            match message.reply_to {
                Some(ref reply_to) => reply_to.get_text().map(|x| x.data.clone()),
                None => None,
            }
        } else {
            Some(String::from(maybe_text))
        };
        let text = if let Some(text) = maybe_text {
            match self.transformer.transform(&text) {
                Ok(text) => {
                    if self.monospace_reply {
                        format!("```\n{}\n```", text)
                    } else {
                        text
                    }
                }
                Err(err) => err.to_string(),
            }
        } else {
            String::from("You should provide some text")
        };
        context
            .get::<MessageSender>()
            .send(&message, text, ReplyTo::Reply)
    }
}
