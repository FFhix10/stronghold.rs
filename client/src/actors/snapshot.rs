// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::type_complexity)]

use actix::{dev::MessageResponse, Actor, Handler, Message, Supervised};

use engine::{
    snapshot,
    vault::{ClientId, DbView, Key, VaultId},
};

use crate::{
    internals,
    state::{
        secure::Store,
        snapshot::{ReadError, Snapshot, SnapshotState, WriteError},
    },
    Provider,
};
use std::collections::HashMap;

/// re-export local modules
pub use messages::*;
pub use returntypes::*;

pub mod returntypes {

    use super::*;

    /// Return type for loaded snapshot file
    #[derive(MessageResponse)]
    pub struct ReturnReadSnapshot {
        pub id: ClientId,

        pub data: Box<(
            HashMap<VaultId, Key<internals::Provider>>,
            DbView<internals::Provider>,
            Store,
        )>,
    }
}

pub mod messages {

    use crate::state::snapshot::SnapshotFile;

    use super::*;

    pub struct WriteSnapshot {
        pub key: snapshot::Key,
        pub snapshot_file: SnapshotFile,
    }

    impl Message for WriteSnapshot {
        type Result = Result<(), WriteError>;
    }

    pub struct FillSnapshot {
        pub data: Box<(HashMap<VaultId, Key<Provider>>, DbView<Provider>, Store)>,
        pub id: ClientId,
    }

    impl Message for FillSnapshot {
        type Result = ();
    }

    #[derive(Default)]
    pub struct ReadFromSnapshot {
        pub key: snapshot::Key,
        pub snapshot_file: SnapshotFile,
        pub id: ClientId,
        pub fid: Option<ClientId>,
    }

    impl Message for ReadFromSnapshot {
        type Result = Result<returntypes::ReturnReadSnapshot, ReadError>;
    }

    #[derive(Default)]
    pub struct ActorStateFromSnapshot {
        pub id: ClientId,
    }

    impl Message for ActorStateFromSnapshot {
        type Result = returntypes::ReturnReadSnapshot;
    }

    #[derive(Default)]
    pub struct LoadFromDisk {
        pub key: snapshot::Key,
        pub snapshot_file: SnapshotFile,
    }

    impl Message for LoadFromDisk {
        type Result = Result<(), ReadError>;
    }
}

impl Actor for Snapshot {
    type Context = actix::Context<Self>;
}

// actix impl
impl Supervised for Snapshot {}

impl Handler<messages::FillSnapshot> for Snapshot {
    type Result = ();

    fn handle(&mut self, msg: messages::FillSnapshot, _ctx: &mut Self::Context) -> Self::Result {
        self.state.add_data(msg.id, *msg.data);
    }
}

impl Handler<messages::ReadFromSnapshot> for Snapshot {
    type Result = Result<returntypes::ReturnReadSnapshot, ReadError>;

    /// This will try to read from a snapshot on disk, otherwise load from a local snapshot
    /// in memory. Returns the loaded snapshot data, that must be loaded inside the client
    /// for access.
    fn handle(&mut self, msg: messages::ReadFromSnapshot, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.fid.unwrap_or(msg.id);

        if self.has_data(id) {
            let data = self.get_state(id);

            Ok(ReturnReadSnapshot {
                id,
                data: Box::new(data),
            })
        } else {
            let mut snapshot = Snapshot::read_from_snapshot(msg.snapshot_file, msg.key)?;
            let data = snapshot.get_state(id);
            *self = snapshot;

            Ok(ReturnReadSnapshot {
                id,
                data: Box::new(data),
            })
        }
    }
}

impl Handler<messages::LoadFromDisk> for Snapshot {
    type Result = Result<(), ReadError>;

    fn handle(&mut self, msg: messages::LoadFromDisk, _ctx: &mut Self::Context) -> Self::Result {
        let snapshot = Snapshot::read_from_snapshot(msg.snapshot_file, msg.key)?;
        *self = snapshot;

        Ok(())
    }
}

impl Handler<messages::ActorStateFromSnapshot> for Snapshot {
    type Result = returntypes::ReturnReadSnapshot;

    fn handle(&mut self, msg: messages::ActorStateFromSnapshot, _ctx: &mut Self::Context) -> Self::Result {
        let data = self.get_state(msg.id);

        ReturnReadSnapshot {
            id: msg.id,
            data: Box::new(data),
        }
    }
}

impl Handler<messages::WriteSnapshot> for Snapshot {
    type Result = Result<(), WriteError>;

    fn handle(&mut self, msg: messages::WriteSnapshot, _ctx: &mut Self::Context) -> Self::Result {
        self.write_to_snapshot(msg.snapshot_file, msg.key)?;

        self.state = SnapshotState::default();

        Ok(())
    }
}
