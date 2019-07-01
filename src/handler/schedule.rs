use crate::store::schedule::ScheduleStore;
use carapax::core::{
    Api,
    {methods::SendMessage, types::Integer},
};
use futures::{
    future::{self, Either},
    Future, Stream,
};

pub fn scheduled_handler(chat_id: Integer, api: Api, store: ScheduleStore) {
    for item in store {
        let api_clone = api.clone();
        api.spawn(item.get_interval().for_each(move |_| {
            if let Some(text) = item.get_random_message() {
                Either::A(
                    api_clone
                        .execute(SendMessage::new(chat_id, text.clone()))
                        .then(move |r| {
                            if let Err(err) = r {
                                log::error!(
                                    "Failed to send scheduled message ({:?}): {:?}",
                                    text,
                                    err
                                );
                            }
                            Ok(())
                        }),
                )
            } else {
                log::warn!("No scheduled message found for item: {:?}", item);
                Either::B(future::ok(()))
            }
        }));
    }
}
