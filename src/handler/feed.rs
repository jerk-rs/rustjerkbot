use crate::store::{db::Store, feed::FeedStore};
use carapax::core::{
    Api,
    {
        methods::SendMessage,
        types::{Integer, ParseMode},
    },
};
use failure::Error;
use futures::{
    future::{self, Either},
    Future, Stream,
};

pub fn feed_handler(chat_id: Integer, api: Api, db_store: Store, store: FeedStore) {
    for channel in store {
        let api_clone = api.clone();
        let db_clone = db_store.clone();
        api.spawn(
            channel
                .get_interval()
                .map_err(Error::from)
                .for_each(move |_| {
                    let api_clone = api_clone.clone();
                    let db_clone = db_clone.clone();
                    channel.get_last_item().and_then(move |item| {
                        if let Some(item) = item {
                            Either::A(db_clone.has_feed_entry(&item).and_then(move |has_entry| {
                                if has_entry {
                                    Either::A(future::ok(()))
                                } else {
                                    Either::B(
                                        api_clone
                                            .execute(
                                                SendMessage::new(chat_id, item.clone())
                                                    .parse_mode(ParseMode::Html),
                                            )
                                            .then(move |r| {
                                                if let Err(err) = r {
                                                    log::error!(
                                                        "Failed to send feed message ({:?}): {:?}",
                                                        item,
                                                        err
                                                    );
                                                    Either::A(future::ok(()))
                                                } else {
                                                    Either::B(db_clone.track_feed_entry(&item))
                                                }
                                            }),
                                    )
                                }
                            }))
                        } else {
                            Either::B(future::ok(()))
                        }
                    })
                }),
        )
    }
}
