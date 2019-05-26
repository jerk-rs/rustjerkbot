use crate::{
    config::Config,
    shippering::{
        future::ShipperingFuture,
        template::{TemplateKind, TemplateStore},
    },
    store::Store,
    utils::futures_ordered,
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
use failure::Error;
use futures::{
    future::{self, Either},
    Future, Stream,
};
use std::{sync::Arc, time::Duration};
use tokio_timer::throttle::Throttle;

pub struct ShipperingHandler {
    store: Arc<TemplateStore>,
}

impl ShipperingHandler {
    pub fn new(store: TemplateStore) -> Self {
        Self {
            store: Arc::new(store),
        }
    }
}

impl CommandHandler for ShipperingHandler {
    type Output = HandlerFuture;

    fn handle(&self, context: &mut Context, message: Message, _args: Vec<String>) -> Self::Output {
        let api = context.get::<Api>().clone();
        let (pair_timeout, message_timeout) = {
            let config = context.get::<Config>();
            (
                config.shippering_pair_timeout,
                config.shippering_message_timeout,
            )
        };
        let store = context.get::<Store>().clone();
        let tpl_store = self.store.clone();
        let chat_id = message.get_chat_id();
        let parse_mode = ParseMode::Html;

        HandlerFuture::new(
            ShipperingFuture::new(api.clone(), store.clone(), chat_id, pair_timeout).and_then(
                move |pair| -> Box<Future<Item = HandlerResult, Error = Error> + Send> {
                    match pair {
                        Some(pair) => {
                            if pair.is_old() {
                                Box::new(
                                    future::result(tpl_store.render_template(
                                        TemplateKind::Found,
                                        &pair.active_user().custom_mention("актив"),
                                        &pair.passive_user().custom_mention("пассив"),
                                    ))
                                    .and_then(
                                        move |message| {
                                            if let Some(message) = message {
                                                Either::A(
                                                    api.execute(
                                                        SendMessage::new(chat_id, message)
                                                            .parse_mode(parse_mode),
                                                    )
                                                    .map(|_| HandlerResult::Continue),
                                                )
                                            } else {
                                                Either::B(
                                                    api.execute(SendMessage::new(
                                                        chat_id,
                                                        "Template for existing pair not found",
                                                    ))
                                                    .map(|_| HandlerResult::Continue),
                                                )
                                            }
                                        },
                                    ),
                                )
                            } else {
                                Box::new(
                                    future::result(tpl_store.render_template(
                                        TemplateKind::New,
                                        &pair.active_user().mention(),
                                        &pair.passive_user().mention(),
                                    ))
                                    .and_then(move |result| {
                                        if let Some(result) = result {
                                            Either::A(
                                                Throttle::new(
                                                    futures_ordered(result.lines().map(|line| {
                                                        api.execute(
                                                            SendMessage::new(chat_id, line)
                                                                .parse_mode(parse_mode),
                                                        )
                                                        .map_err(|e| e.compat())
                                                    })),
                                                    Duration::from_secs(message_timeout),
                                                )
                                                .collect()
                                                .from_err()
                                                .map(|_| HandlerResult::Continue),
                                            )
                                        } else {
                                            Either::B(
                                                api.execute(SendMessage::new(
                                                    chat_id,
                                                    "Template for new pair not found",
                                                ))
                                                .map(|_| HandlerResult::Continue),
                                            )
                                        }
                                    }),
                                )
                            }
                        }
                        None => Box::new(
                            api.execute(
                                SendMessage::new(chat_id, tpl_store.get_not_found_message())
                                    .parse_mode(parse_mode),
                            )
                            .map(|_| HandlerResult::Continue),
                        ),
                    }
                },
            ),
        )
    }
}
