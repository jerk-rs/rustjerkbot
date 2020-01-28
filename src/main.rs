use carapax::{longpoll::LongPoll, webhook, Api, CommandDispatcher, Dispatcher};
use carapax_access::{AccessHandler, AccessRule, InMemoryAccessPolicy};
use carapax_session::{backend::redis::RedisBackend as RedisSessionBackend, SessionCollector, SessionManager};
use darkredis::ConnectionPool as RedisPool;
use dotenv::dotenv;
use env_logger;
use liquid::ParserBuilder as TemplateParserBuilder;
use reqwest::Client as HttpClient;
use std::sync::Arc;
use tokio_postgres::{connect as pg_connect, NoTls as PgNoTls};

const SESSION_NAMESPACE: &str = "rustjerkbot:";

mod config;
mod context;
mod handler;
mod scheduler;
mod sender;
mod shippering;
mod syndication;

use self::{
    config::Config,
    context::Context,
    handler::{
        autoresponse::AutoresponseHandler,
        ferris::handle_ferris,
        greetings::handle_new_chat_member,
        text::{replace_text_handler, TransformCommand},
        tracker::track_chat_member,
        user::get_user_info,
    },
    scheduler::Scheduler,
    sender::MessageSender,
    shippering::Shippering,
    syndication::Syndication,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().expect("Can not read config");
    let api_config = config.get_api_config().expect("Can not get API config");
    let api = Api::new(api_config).expect("Failed to create API");

    let (pg_client, pg_connection) = pg_connect(&config.postgres_url, PgNoTls)
        .await
        .expect("PostgreSQL connection failed");
    tokio::spawn(async move {
        if let Err(e) = pg_connection.await {
            log::error!("PostgreSQL connection error: {}", e);
        }
    });
    let pg_client = Arc::new(pg_client);

    let session_backend = RedisSessionBackend::new(
        SESSION_NAMESPACE,
        RedisPool::create(config.redis_url.clone(), None, num_cpus::get())
            .await
            .expect("Redis connection failed"),
    );

    let mut session_collector = SessionCollector::new(
        session_backend.clone(),
        config.session_gc_period,
        config.session_gc_timeout,
    );
    tokio::spawn(async move { session_collector.run().await });

    let context = Context {
        api: api.clone(),
        config: config.clone(),
        http_client: HttpClient::new(),
        message_sender: MessageSender::new(api.clone(), SessionManager::new(session_backend)),
        pg_client: pg_client.clone(),
        tpl_parser: TemplateParserBuilder::with_liquid()
            .build()
            .expect("Can not create template parser"),
    };

    let scheduler = Scheduler::new(context.clone());
    scheduler.spawn().await.expect("Failed to spawn messages scheduler");

    let shippering = Shippering::new(context.clone());
    tokio::spawn(shippering.run());

    let syndication = Syndication::new(context.clone());
    tokio::spawn(async move {
        if let Err(err) = syndication.run().await {
            log::error!("syndication error: {}", err);
        }
    });

    let mut command_dispatcher = CommandDispatcher::new();
    command_dispatcher.add_handler("/arrow", TransformCommand::arrow());
    command_dispatcher.add_handler("/cw", TransformCommand::cw());
    command_dispatcher.add_handler("/jerkify", TransformCommand::jerkify());
    command_dispatcher.add_handler("/huify", TransformCommand::huify());
    command_dispatcher.add_handler("/reverse", TransformCommand::reverse());
    command_dispatcher.add_handler("/square", TransformCommand::square());
    command_dispatcher.add_handler("/star", TransformCommand::star());
    command_dispatcher.add_handler("/user", get_user_info);

    let mut dispatcher = Dispatcher::new(context);
    dispatcher.add_handler(AccessHandler::new(
        InMemoryAccessPolicy::default().push_rule(AccessRule::allow_chat(config.chat_id)),
    ));
    dispatcher.add_handler(track_chat_member);
    dispatcher.add_handler(handle_new_chat_member);
    dispatcher.add_handler(
        AutoresponseHandler::new(pg_client)
            .await
            .expect("Failed to create autoresponse handler"),
    );
    dispatcher.add_handler(replace_text_handler);
    dispatcher.add_handler(command_dispatcher);
    dispatcher.add_handler(handle_ferris);

    match config.webhook_url {
        Some((addr, path)) => {
            log::info!("Starting receiving updates via webhook: {}{}", addr, path);
            webhook::run_server(addr, path, dispatcher)
                .await
                .expect("Failed to run webhook server");
        }
        None => {
            log::info!("Starting receiving updates via long polling");
            LongPoll::new(api, dispatcher).run().await;
        }
    };
}
