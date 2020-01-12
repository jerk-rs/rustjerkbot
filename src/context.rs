use crate::{config::Config, sender::MessageSender};
use carapax::Api;
use liquid::Parser as TemplateParser;
use reqwest::Client as HttpClient;
use std::sync::Arc;
use tokio_postgres::Client as PgClient;

#[derive(Clone)]
pub struct Context {
    pub api: Api,
    pub config: Config,
    pub http_client: HttpClient,
    pub message_sender: MessageSender,
    pub pg_client: Arc<PgClient>,
    pub tpl_parser: TemplateParser,
}
