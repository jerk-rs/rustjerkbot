use atom_syndication::Feed as AtomFeed;
use failure::{format_err, Error};
use futures::{
    future::{self, Either},
    Future, Stream,
};
use reqwest::{r#async::Client, IntoUrl, Url};
use rss::Channel as RssChannel;
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    time::Duration,
    vec::IntoIter as VecIntoIter,
};
use tokio_timer::Interval;

#[derive(Copy, Clone, Debug, Deserialize)]
enum ChannelKind {
    #[serde(rename = "rss")]
    Rss,
    #[serde(rename = "atom")]
    Atom,
}

#[derive(Debug, Deserialize)]
struct ChannelConfig {
    url: String,
    interval: u64,
    kind: ChannelKind,
}

#[derive(Debug, Deserialize)]
struct FeedConfig {
    channels: Vec<ChannelConfig>,
}

impl FeedConfig {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut f = File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        Ok(serde_yaml::from_slice(&buf)?)
    }
}

pub struct FeedStore {
    items: Vec<ChannelStore>,
}

impl FeedStore {
    pub fn new<P: AsRef<Path>>(config_path: P) -> Result<Self, Error> {
        let config = FeedConfig::from_path(config_path)?;
        let client = Client::new();
        let mut items = Vec::with_capacity(config.channels.len());
        for i in config.channels {
            items.push(ChannelStore::new(
                client.clone(),
                &i.url,
                i.kind,
                i.interval,
            )?)
        }
        Ok(Self { items })
    }
}

pub struct ChannelStore {
    url: Url,
    interval: u64,
    client: Client,
    kind: ChannelKind,
}

impl ChannelStore {
    fn new<U: IntoUrl>(
        client: Client,
        url: U,
        kind: ChannelKind,
        interval: u64,
    ) -> Result<Self, Error> {
        Ok(Self {
            url: url.into_url()?,
            interval: interval,
            kind,
            client,
        })
    }

    pub fn get_interval(&self) -> Interval {
        Interval::new_interval(Duration::from_secs(self.interval))
    }

    pub fn get_last_item(&self) -> impl Future<Item = Option<String>, Error = Error> {
        let kind = self.kind;
        let url = self.url.clone();
        self.client
            .get(url.clone())
            .send()
            .from_err()
            .and_then(move |rep| {
                if rep.status().is_success() {
                    Either::A(rep.into_body().concat2().map_err(Error::from).and_then(
                        move |body| {
                            let body = body.into_iter().collect::<Vec<u8>>();
                            let buf = BufReader::new(body.as_slice());
                            match kind {
                                ChannelKind::Rss => Either::A(
                                    future::result(RssChannel::read_from(buf).map(|channel| {
                                        let items = channel.into_items();
                                        if items.is_empty() {
                                            None
                                        } else {
                                            let item = &items[0];
                                            match (item.title(), item.link()) {
                                                (Some(title), Some(link)) => Some(format!(
                                                    r#"<a href="{}">{}</a>"#,
                                                    link,
                                                    ammonia::clean(title),
                                                )),
                                                _ => None,
                                            }
                                        }
                                    }))
                                    .map_err(Error::from),
                                ),
                                ChannelKind::Atom => Either::B(
                                    future::result(AtomFeed::read_from(buf).map(|feed| {
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
                                                let title = link.title().unwrap_or(entry.title());
                                                Some(format!(
                                                    r#"<a href="{}">{}</a>"#,
                                                    link.href(),
                                                    ammonia::clean(title),
                                                ))
                                            }
                                        }
                                    }))
                                    .map_err(Error::from),
                                ),
                            }
                        },
                    ))
                } else {
                    Either::B(future::err(format_err!(
                        "Failed to get feed from URL: {}, status={}",
                        url,
                        rep.status()
                    )))
                }
            })
    }
}

impl IntoIterator for FeedStore {
    type Item = ChannelStore;
    type IntoIter = VecIntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
