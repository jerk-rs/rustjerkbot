use crate::{
    handler::shippering::state::ShipperingState,
    store::db::{Pair, Store},
};
use carapax::core::{
    types::{Integer, ResponseError},
    Api,
};
use failure::Error;
use futures::{try_ready, Async, Future, Poll};
use rand::{seq::SliceRandom, thread_rng};

/// A future used to find a pair
#[must_use = "futures do nothing unless polled"]
pub(super) struct ShipperingFuture {
    api: Api,
    db_store: Store,
    state: ShipperingState,
    chat_id: Integer,
}

impl ShipperingFuture {
    pub(super) fn new(api: Api, db_store: Store, chat_id: Integer, pair_timeout: u64) -> Self {
        let state = ShipperingState::FindPair(Box::new(db_store.find_last_pair(pair_timeout)));
        Self {
            api,
            db_store,
            state,
            chat_id,
        }
    }
}

impl Future for ShipperingFuture {
    type Item = Option<Pair>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.state {
                ShipperingState::FindPair(ref mut f) => {
                    let maybe_pair = try_ready!(f.poll());
                    if let Some(pair) = maybe_pair {
                        return Ok(Async::Ready(Some(pair)));
                    } else {
                        self.state.switch_to_get_users(&self.db_store);
                    }
                }
                ShipperingState::GetUsers(ref mut f) => {
                    let maybe_pair: Vec<Integer> = try_ready!(f.poll())
                        .choose_multiple(&mut thread_rng(), 2)
                        .cloned()
                        .collect();
                    if maybe_pair.len() == 2 {
                        let (active_user_id, passive_user_id) = (maybe_pair[0], maybe_pair[1]);
                        self.state.switch_to_get_active_member(
                            &self.api,
                            self.chat_id,
                            active_user_id,
                            passive_user_id,
                        );
                    } else {
                        return Ok(Async::Ready(None));
                    }
                }
                ShipperingState::GetActiveMember {
                    active_user_id,
                    passive_user_id,
                    future: ref mut f,
                } => {
                    let is_member = match f.poll() {
                        Ok(Async::Ready(chat_member)) => chat_member.is_member(),
                        Ok(Async::NotReady) => return Ok(Async::NotReady),
                        Err(e) => {
                            if let Some(ResponseError {
                                error_code: Some(400),
                                ..
                            }) = e.downcast_ref::<ResponseError>()
                            {
                                false
                            } else {
                                return Err(e);
                            }
                        }
                    };
                    if is_member {
                        self.state.switch_to_get_passive_member(
                            &self.api,
                            self.chat_id,
                            active_user_id,
                            passive_user_id,
                        );
                    } else {
                        self.state
                            .switch_to_del_users(&self.db_store, &[active_user_id]);
                    }
                }
                ShipperingState::GetPassiveMember {
                    active_user_id,
                    passive_user_id,
                    future: ref mut f,
                } => {
                    let is_member = match f.poll() {
                        Ok(Async::Ready(chat_member)) => chat_member.is_member(),
                        Ok(Async::NotReady) => return Ok(Async::NotReady),
                        Err(e) => {
                            if let Some(ResponseError {
                                error_code: Some(400),
                                ..
                            }) = e.downcast_ref::<ResponseError>()
                            {
                                false
                            } else {
                                return Err(e);
                            }
                        }
                    };
                    if is_member {
                        self.state.switch_to_load_pair(
                            &self.db_store,
                            active_user_id,
                            passive_user_id,
                        );
                    } else {
                        self.state
                            .switch_to_del_users(&self.db_store, &[passive_user_id]);
                    }
                }
                ShipperingState::DelUsers(ref mut f) => {
                    try_ready!(f.poll());
                    self.state.switch_to_get_users(&self.db_store);
                }
                ShipperingState::LoadPair(ref mut f) => {
                    let pair = try_ready!(f.poll());
                    match pair {
                        (Some(active_user), Some(passive_user)) => {
                            self.state.switch_to_save_pair(
                                &self.db_store,
                                Pair::new(active_user, passive_user)?,
                            );
                        }
                        _ => return Ok(Async::Ready(None)),
                    }
                }
                ShipperingState::SavePair(ref mut f) => {
                    let pair = try_ready!(f.poll());
                    return Ok(Async::Ready(Some(pair)));
                }
            }
        }
    }
}
