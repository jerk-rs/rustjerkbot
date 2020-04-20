use crate::{
    context::Context,
    handler::{
        autoresponse::AutoresponseHandler,
        ferris::handle_ferris,
        greetings::handle_new_chat_member,
        text::{replace_text_handler, TransformCommand},
        user::get_user_info,
    },
};
use carapax::{
    access::{AccessHandler, AccessRule, InMemoryAccessPolicy},
    types::Integer,
    Dispatcher,
};

pub async fn create(context: Context, chat_id: Integer) -> Dispatcher<Context> {
    let pg_client = context.pg_client.clone();
    let mut dispatcher = Dispatcher::new(context);
    dispatcher.add_handler(AccessHandler::new(
        InMemoryAccessPolicy::default().push_rule(AccessRule::allow_chat(chat_id)),
    ));
    dispatcher.add_handler(handle_new_chat_member);
    dispatcher.add_handler(
        AutoresponseHandler::new(pg_client)
            .await
            .expect("Failed to create autoresponse handler"),
    );
    dispatcher.add_handler(replace_text_handler);
    dispatcher.add_handler(TransformCommand::arrow());
    dispatcher.add_handler(TransformCommand::cw());
    dispatcher.add_handler(TransformCommand::jerkify());
    dispatcher.add_handler(TransformCommand::huify());
    dispatcher.add_handler(TransformCommand::reverse());
    dispatcher.add_handler(TransformCommand::square());
    dispatcher.add_handler(TransformCommand::star());
    dispatcher.add_handler(get_user_info);
    dispatcher.add_handler(handle_ferris);
    dispatcher
}
