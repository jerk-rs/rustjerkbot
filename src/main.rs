use carapax::{
    longpoll::LongPoll,
    session::{backend::redis::RedisBackend as RedisSessionBackend, SessionCollector, SessionManager},
    webhook, Api,
};
use darkredis::ConnectionPool as RedisPool;
use dotenv::dotenv;
use env_logger;
use reqwest::Client as HttpClient;
use std::{env, sync::Arc, time::Duration};
use tokio_postgres::{connect as pg_connect, NoTls as PgNoTls};

const SESSION_NAMESPACE: &str = "rustjerkbot:";
const SESSION_GC_PERIOD: Duration = Duration::from_secs(3600);
const SESSION_GC_TIMEOUT: Duration = Duration::from_secs(604_800);

mod config;
mod context;
mod db;
mod dispatcher;
mod handler;
mod scheduler;
mod sender;
mod syndication;

use self::{config::Config, context::Context, scheduler::Scheduler, sender::MessageSender, syndication::Syndication};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().expect("Can not read config");

    let (mut pg_client, pg_connection) = pg_connect(&config.postgres_url, PgNoTls)
        .await
        .expect("PostgreSQL connection failed");
    tokio::spawn(async move {
        if let Err(e) = pg_connection.await {
            log::error!("PostgreSQL connection error: {}", e);
        }
    });

    let mut args = env::args();
    match args.nth(1) {
        Some(command) => match command.as_str() {
            "migrate" => {
                db::migrations::runner()
                    .run_async(&mut pg_client)
                    .await
                    .expect("Failed to run migrations");
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        },
        None => {
            let api_config = config.get_api_config().expect("Can not get API config");
            let api = Api::new(api_config).expect("Failed to create API");

            let pg_client = Arc::new(pg_client);

            let session_backend = RedisSessionBackend::new(
                SESSION_NAMESPACE,
                RedisPool::create(config.redis_url.clone(), None, num_cpus::get())
                    .await
                    .expect("Redis connection failed"),
            );

            let mut session_collector =
                SessionCollector::new(session_backend.clone(), SESSION_GC_PERIOD, SESSION_GC_TIMEOUT);
            tokio::spawn(async move { session_collector.run().await });

            let context = Context {
                api: api.clone(),
                config: config.clone(),
                http_client: HttpClient::new(),
                message_sender: MessageSender::new(api.clone(), SessionManager::new(session_backend)),
                pg_client: pg_client.clone(),
            };

            let scheduler = Scheduler::new(context.clone());
            scheduler.spawn().await.expect("Failed to spawn messages scheduler");

            let syndication = Syndication::new(context.clone());
            tokio::spawn(async move {
                if let Err(err) = syndication.run().await {
                    log::error!("syndication error: {}", err);
                }
            });

            let dispatcher = dispatcher::create(context, config.chat_id).await;

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
    };
}
