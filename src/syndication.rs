use crate::context::Context;
use atom_syndication::{Error as AtomError, Feed as AtomFeed};
use bytes::buf::BufExt;
use carapax::{methods::SendMessage, types::ParseMode, ExecuteError};
use reqwest::{Error as HttpError, StatusCode};
use rss::{Channel as RssChannel, Error as RssError};
use std::{error::Error, fmt, str::FromStr, time::Duration};
use tokio::time::delay_for;
use tokio_postgres::Error as PostgresError;

pub struct Syndication {
    context: Context,
}

impl Syndication {
    pub fn new(context: Context) -> Self {
        Self { context }
    }

    async fn get_feeds(&self) -> Result<Vec<Feed>, SyndicationError> {
        let mut result = Vec::new();
        let rows = self
            .context
            .pg_client
            .query(
                "SELECT id, url, kind, last_entry
                FROM feeds
                WHERE extract(epoch from (now() - last_update)) >= timeout
                OR last_update IS NULL",
                &[],
            )
            .await
            .map_err(SyndicationError::GetFeeds)?;
        for row in rows {
            let id: i32 = row.get(0);
            let url: String = row.get(1);
            let kind: String = row.get(2);
            let last_entry: Option<String> = row.get(3);
            result.push(Feed {
                id,
                url,
                kind: kind.parse()?,
                last_entry,
            })
        }
        Ok(result)
    }

    async fn get_last_entry(&self, url: &str, kind: FeedKind) -> Result<Option<String>, SyndicationError> {
        let rep = self.context.http_client.get(url).send().await?;
        let status = rep.status();
        if !status.is_success() {
            return Err(SyndicationError::BadStatus(status));
        }
        let data = rep.bytes().await?;
        Ok(match kind {
            FeedKind::Rss => {
                let channel = RssChannel::read_from(data.reader())?;
                let items = channel.into_items();
                if items.is_empty() {
                    None
                } else {
                    let item = &items[0];
                    match (item.title(), item.link()) {
                        (Some(title), Some(link)) => {
                            Some(format!(r#"<a href="{}">{}</a>"#, link, ParseMode::Html.escape(title),))
                        }
                        _ => None,
                    }
                }
            }
            FeedKind::Atom => {
                let feed = AtomFeed::read_from(data.reader())?;
                let entries = feed.entries();
                if entries.is_empty() {
                    None
                } else {
                    let entry = &entries[0];
                    let links = entry.links();
                    if links.is_empty() {
                        None
                    } else {
                        let link = &links[0];
                        let title = link.title().unwrap_or_else(|| entry.title());
                        Some(format!(
                            r#"<a href="{}">{}</a>"#,
                            link.href(),
                            ParseMode::Html.escape(title),
                        ))
                    }
                }
            }
        })
    }

    pub async fn run(self) -> Result<(), SyndicationError> {
        let timeout = Duration::from_secs(60);
        loop {
            for feed in self.get_feeds().await? {
                let last_entry = self.get_last_entry(&feed.url, feed.kind).await?;
                if let Some(ref last_entry) = last_entry {
                    if feed.last_entry.map(|x| &x != last_entry).unwrap_or(true) {
                        self.context
                            .api
                            .execute(
                                SendMessage::new(self.context.config.chat_id, last_entry).parse_mode(ParseMode::Html),
                            )
                            .await
                            .map_err(SyndicationError::SendMessage)?;
                    }
                }
                self.context
                    .pg_client
                    .execute(
                        "UPDATE feeds SET last_update = now(), last_entry = $2 WHERE id = $1",
                        &[&feed.id, &last_entry],
                    )
                    .await
                    .map_err(SyndicationError::UpdateFeed)?;
            }
            delay_for(timeout).await
        }
    }
}

struct Feed {
    id: i32,
    url: String,
    kind: FeedKind,
    last_entry: Option<String>,
}

enum FeedKind {
    Atom,
    Rss,
}

impl FromStr for FeedKind {
    type Err = SyndicationError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(match raw {
            "atom" => FeedKind::Atom,
            "rss" => FeedKind::Rss,
            _ => return Err(SyndicationError::UnknownFeedKind(String::from(raw))),
        })
    }
}

#[derive(Debug)]
pub enum SyndicationError {
    Atom(AtomError),
    BadStatus(StatusCode),
    GetFeeds(PostgresError),
    HttpRequest(HttpError),
    Rss(RssError),
    SendMessage(ExecuteError),
    UpdateFeed(PostgresError),
    UnknownFeedKind(String),
}

impl From<AtomError> for SyndicationError {
    fn from(err: AtomError) -> Self {
        SyndicationError::Atom(err)
    }
}

impl From<HttpError> for SyndicationError {
    fn from(err: HttpError) -> Self {
        SyndicationError::HttpRequest(err)
    }
}

impl From<RssError> for SyndicationError {
    fn from(err: RssError) -> Self {
        SyndicationError::Rss(err)
    }
}

impl Error for SyndicationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SyndicationError::GetFeeds(err) => Some(err),
            SyndicationError::HttpRequest(err) => Some(err),
            SyndicationError::Rss(err) => Some(err),
            SyndicationError::SendMessage(err) => Some(err),
            SyndicationError::UpdateFeed(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for SyndicationError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SyndicationError::Atom(err) => write!(out, "failed to parse atom feed: {}", err),
            SyndicationError::BadStatus(status) => write!(out, "server repsond with {} status code", status),
            SyndicationError::GetFeeds(err) => write!(out, "failed to get feeds: {}", err),
            SyndicationError::HttpRequest(err) => write!(out, "http request error: {}", err),
            SyndicationError::Rss(err) => write!(out, "failed to parse RSS: {}", err),
            SyndicationError::SendMessage(err) => write!(out, "failed to send message: {}", err),
            SyndicationError::UpdateFeed(err) => write!(out, "failed to update feed: {}", err),
            SyndicationError::UnknownFeedKind(kind) => write!(out, "unknown feed kind: {}", kind),
        }
    }
}
