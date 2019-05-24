use carapax::{
    context::Context,
    core::{types::Update, Api, UpdateMethod, UpdatesStream},
    App, CommandsHandler, FnHandler,
};
use dotenv::dotenv;
use env_logger;
use futures::{future, Future};

mod config;
mod shippering;
mod store;
mod tracker;
mod utils;

use self::{
    config::Config,
    shippering::{handle_shippering, TemplateStore},
    store::Store,
};

fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().expect("Can not read configuration file");
    let api = Api::new(config.get_api_config()).expect("Can not to create Api");

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
                    .add_handler(
                        CommandsHandler::default().add_handler("/shippering", handle_shippering),
                    )
                    .run(api.clone(), UpdateMethod::poll(UpdatesStream::new(api)))
            })
    }));
}
