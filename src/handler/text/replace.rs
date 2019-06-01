use crate::store::db::Store;
use carapax::{
    context::Context,
    core::{
        methods::{EditMessageText, SendMessage},
        types::Message,
        Api,
    },
    HandlerFuture, HandlerResult,
};
use futures::{
    future::{self, Either},
    Future,
};
use sedregex::find_and_replace;

pub fn replace_text_handler(context: &mut Context, message: Message) -> HandlerFuture {
    let source = match message.reply_to {
        Some(ref reply_to) => reply_to.get_text().map(|text| (reply_to.id, text)),
        None => None,
    };
    let api = context.get::<Api>().clone();
    let db_store = context.get::<Store>().clone();
    let chat_id = message.get_chat_id();
    let is_edited = message.is_edited();
    if let (Some(commands), Some((reply_to_id, text))) = (message.get_text(), source) {
        let commands = commands
            .data
            .split('\n')
            .filter_map(complete_command)
            .collect::<Vec<String>>();
        if commands.is_empty() {
            return HandlerResult::Continue.into();
        }
        let (reply_to_id, reply_text) = match find_and_replace(&text.data, commands) {
            Ok(reply_text) => (reply_to_id, reply_text.to_string()),
            Err(err) => (message.id, err.to_string()),
        };

        HandlerFuture::new(if is_edited {
            Either::A(
                db_store
                    .get_tracked_message(message.id)
                    .and_then(move |message_id| {
                        if let Some(message_id) = message_id {
                            Either::A(
                                api.execute(EditMessageText::new(chat_id, message_id, reply_text))
                                    .map(|_| HandlerResult::Continue),
                            )
                        } else {
                            Either::B(future::ok(HandlerResult::Continue))
                        }
                    }),
            )
        } else {
            Either::B(
                api.execute(SendMessage::new(chat_id, reply_text).reply_to_message_id(reply_to_id))
                    .and_then(move |result_message| {
                        db_store
                            .track_message(message.id, result_message.id)
                            .map(|()| HandlerResult::Continue)
                    }),
            )
        })
    } else {
        HandlerResult::Continue.into()
    }
}

fn complete_command(line: &str) -> Option<String> {
    if !(line.starts_with("s/") || line.starts_with("/s/")) {
        return None;
    }
    let out = if line.starts_with('/') {
        &line[1..]
    } else {
        &line
    };
    let n_slashes = out.matches('/').count();
    let n_escaped_slashes = out.matches("\\/").count();
    let mut out = String::from(out);
    if (n_slashes - n_escaped_slashes) == 2 {
        out.push('/');
    }
    Some(out)
}
