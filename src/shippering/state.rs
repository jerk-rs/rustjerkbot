use crate::store::{Pair, Store, UserData};
use carapax::core::{
    methods::GetChatMember,
    types::{ChatMember, Integer},
    Api,
};
use failure::Error;
use futures::Future;
use std::mem;

type MaybePair = (Option<UserData>, Option<UserData>);

pub(super) enum ShipperingState {
    /// Find a last pair if exists
    FindPair(Box<Future<Item = Option<Pair>, Error = Error> + Send>),
    /// Load all available user IDs from a store
    GetUsers(Box<Future<Item = Vec<Integer>, Error = Error> + Send>),
    /// Get chat member for active from Telegram
    GetActiveMember {
        active_user_id: Integer,
        passive_user_id: Integer,
        future: Box<Future<Item = ChatMember, Error = Error> + Send>,
    },
    /// Get chat member for passive from Telegram
    GetPassiveMember {
        active_user_id: Integer,
        passive_user_id: Integer,
        future: Box<Future<Item = ChatMember, Error = Error> + Send>,
    },
    /// Delete non-members from store
    DelUsers(Box<Future<Item = (), Error = Error> + Send>),
    /// Load data for each user in pair
    LoadPair(Box<Future<Item = MaybePair, Error = Error> + Send>),
    /// Save pair to store
    SavePair(Box<Future<Item = Pair, Error = Error> + Send>),
}

impl ShipperingState {
    pub(super) fn switch_to_get_users(&mut self, store: &Store) {
        mem::replace(
            self,
            ShipperingState::GetUsers(Box::new(store.get_user_ids())),
        );
    }

    pub(super) fn switch_to_get_active_member(
        &mut self,
        api: &Api,
        chat_id: Integer,
        active_user_id: Integer,
        passive_user_id: Integer,
    ) {
        mem::replace(
            self,
            ShipperingState::GetActiveMember {
                active_user_id,
                passive_user_id,
                future: Box::new(api.execute(GetChatMember::new(chat_id, active_user_id))),
            },
        );
    }

    pub(super) fn switch_to_get_passive_member(
        &mut self,
        api: &Api,
        chat_id: Integer,
        active_user_id: Integer,
        passive_user_id: Integer,
    ) {
        mem::replace(
            self,
            ShipperingState::GetPassiveMember {
                active_user_id,
                passive_user_id,
                future: Box::new(api.execute(GetChatMember::new(chat_id, passive_user_id))),
            },
        );
    }

    pub(super) fn switch_to_del_users(&mut self, store: &Store, user_ids: &[Integer]) {
        mem::replace(
            self,
            ShipperingState::DelUsers(Box::new(store.del_users(user_ids))),
        );
    }

    pub(super) fn switch_to_save_pair(&mut self, store: &Store, pair: Pair) {
        mem::replace(
            self,
            ShipperingState::SavePair(Box::new(store.save_pair(pair))),
        );
    }

    pub(super) fn switch_to_load_pair(
        &mut self,
        store: &Store,
        active_user_id: Integer,
        passive_user_id: Integer,
    ) {
        mem::replace(
            self,
            ShipperingState::LoadPair(Box::new(
                store
                    .get_user(active_user_id)
                    .join(store.get_user(passive_user_id)),
            )),
        );
    }
}
