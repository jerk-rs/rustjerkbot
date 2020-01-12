use self::{
    arrow::Arrow, base::TransformText, chain::Chain, cw::Cw, huify::Huify, reverse::Reverse, square::Square, star::Star,
};
use crate::{
    context::Context,
    sender::{ReplyTo, SendError},
};
use carapax::{async_trait, Command, Handler};

mod arrow;
mod base;
mod chain;
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

impl TransformCommand<Chain> {
    pub fn jerkify() -> Self {
        Self {
            transformer: Chain::new(vec![
                Box::new(Huify::new()),
                Box::new(Reverse),
                Box::new(Huify::new()),
                Box::new(Reverse),
            ]),
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

#[async_trait]
impl<T> Handler<Context> for TransformCommand<T>
where
    T: TransformText + Send,
{
    type Input = Command;
    type Output = Result<(), SendError>;

    async fn handle(&mut self, context: &Context, input: Self::Input) -> Self::Output {
        let maybe_text = input.get_args().join(" ");
        let maybe_text = maybe_text.trim();
        let message = input.get_message();
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
                        format!("<pre>{}\n</pre>", text)
                    } else {
                        text
                    }
                }
                Err(err) => err.to_string(),
            }
        } else {
            String::from("You should provide some text")
        };
        context.message_sender.send(&message, text, ReplyTo::Reply).await
    }
}
