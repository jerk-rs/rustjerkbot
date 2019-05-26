use carapax::{
    context::Context,
    core::{types::Update, Api, UpdateMethod, UpdatesStream},
    App, CommandsHandler, FnHandler,
};
use dotenv::dotenv;
use env_logger;
use futures::{future, Future};

mod autoresponse;
mod config;
mod shippering;
mod store;
mod text;
mod tracker;
mod utils;

use self::{
    autoresponse::{AutoresponseHandler, MessageStore},
    config::Config,
    shippering::{handle_shippering, TemplateStore},
    store::Store,
    text::TransformCommand,
};

fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().expect("Can not read configuration file");
    let api = Api::new(config.get_api_config()).expect("Can not to create Api");

    let msg_store =
        MessageStore::from_file("data/messages.yml").expect("Failed to create message store");

    let mut tpl_store = TemplateStore::new().expect("Failed to create template store");
    tpl_store
        .load_file("data/shippering")
        .expect("Unable to load shippering templates");

    tokio::run(future::lazy(move || {
        Store::open(config.redis_url.clone())
            .map_err(|e| log::error!("Unable to open store: {:?}", e))
            .and_then(|store| {
                let setup_context = move |context: &mut Context, _update: Update| {
                    context.set(store.clone());
                    context.set(config.clone());
                    context.set(tpl_store.clone());
                };
                App::new()
                    .add_handler(FnHandler::from(setup_context))
                    .add_handler(FnHandler::from(tracker::handle_update))
                    .add_handler(AutoresponseHandler::new(msg_store))
                    .add_handler(
                        CommandsHandler::default()
                            .add_handler("/shippering", handle_shippering)
                            .add_handler("/arrow", TransformCommand::new(text::transform::to_arrow))
                            .add_handler(
                                "/huify",
                                TransformCommand::new(text::transform::to_huified)
                                    .without_monospace_reply(),
                            )
                            .add_handler(
                                "/square",
                                TransformCommand::new(text::transform::to_square),
                            )
                            .add_handler("/star", TransformCommand::new(text::transform::to_star))
                            .add_handler("/cw", TransformCommand::new(text::transform::to_cw)),
                    )
                    .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api)))
            })
    }));
}
