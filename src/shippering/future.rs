use crate::{
    shippering::state::ShipperingState,
    store::{Pair, Store},
};
use carapax::core::{types::Integer, Api};
use failure::Error;
use futures::{try_ready, Async, Future, Poll};
use rand::{seq::SliceRandom, thread_rng};

/// A future used to find a pair
#[must_use = "futures do nothing unless polled"]
pub(super) struct ShipperingFuture {
    api: Api,
    store: Store,
    state: ShipperingState,
    chat_id: Integer,
}

impl ShipperingFuture {
    pub(super) fn new(api: Api, store: Store, chat_id: Integer, pair_timeout: u64) -> Self {
        let state = ShipperingState::FindPair(Box::new(store.find_last_pair(pair_timeout)));
        Self {
            api,
            store,
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
                        self.state.switch_to_get_users(&self.store);
                    }
                }
                ShipperingState::GetUsers(ref mut f) => {
                    let maybe_pair: Vec<Integer> = try_ready!(f.poll())
                        .choose_multiple(&mut thread_rng(), 2)
                        .cloned()
                        .collect();
                    if maybe_pair.len() == 2 {
                        let (active_user_id, passive_user_id) = (maybe_pair[0], maybe_pair[1]);
                        self.state.switch_to_get_members(
                            &self.api,
                            self.chat_id,
                            active_user_id,
                            passive_user_id,
                        );
                    } else {
                        return Ok(Async::Ready(None));
                    }
                }
                ShipperingState::GetMembers(ref mut f) => {
                    let (active_member, passive_member) = try_ready!(f.poll());
                    let has_active = active_member.is_member();
                    let has_passive = passive_member.is_member();
                    let active_user_id = active_member.user().id;
                    let passive_user_id = passive_member.user().id;
                    if has_active && has_passive {
                        self.state.switch_to_load_pair(
                            &self.store,
                            active_user_id,
                            passive_user_id,
                        );
                    } else {
                        let mut user_ids = vec![];
                        if !has_active {
                            user_ids.push(active_user_id);
                        }
                        if !has_passive {
                            user_ids.push(passive_user_id);
                        }
                        assert!(!user_ids.is_empty());
                        self.state.switch_to_del_users(&self.store, &user_ids);
                    }
                }
                ShipperingState::DelUsers(ref mut f) => {
                    try_ready!(f.poll());
                    self.state.switch_to_get_users(&self.store);
                }
                ShipperingState::LoadPair(ref mut f) => {
                    let pair = try_ready!(f.poll());
                    match pair {
                        (Some(active_user), Some(passive_user)) => {
                            self.state.switch_to_save_pair(
                                &self.store,
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
