use std::{collections::HashMap, sync::Arc};

use crate::storage::keys::*;
use crate::storage::rocksdb::RocksDBEngine;
use bincode::deserialize;
use log::error;
use prost::Message as _;
use raft::eraftpb::HardState;
use raft::prelude::ConfState;
use raft::prelude::SnapshotMetadata;

pub struct RaftMachineStorage {
    pub uncommit_index: HashMap<u64, i8>,
    pub trigger_snap_unavailable: bool,
    pub snapshot_metadata: SnapshotMetadata,
    pub rocksdb_engine_handler: Arc<RocksDBEngine>,
}

impl RaftMachineStorage {
    pub fn new(rocksdb_engine_handler: Arc<RocksDBEngine>) -> Self {
        let uncommit_index = HashMap::new();
        let mut rc = RaftMachineStorage {
            snapshot_metadata: SnapshotMetadata::default(),
            trigger_snap_unavailable: false,
            uncommit_index,
            rocksdb_engine_handler,
        };
        rc.uncommit_index = rc.uncommit_index();
        rc.snapshot_metadata = rc.create_snapshot_metadata();
        rc
    }

    pub fn hard_state(&self) -> HardState {
        let key = key_name_by_hard_state();
        let value = self
            .rocksdb_engine_handler
            .read::<Vec<u8>>(self.rocksdb_engine_handler.cf_cluster(), &key)
            .unwrap();
        if value.is_none() {
            HardState::default()
        } else {
            return HardState::decode(value.as_ref().unwrap());
        }
    }

    pub fn conf_state(&self) -> ConfState {
        let key = key_name_by_conf_state();
        let value = self
            .rocksdb_engine_handler
            .read::<Vec<u8>>(self.rocksdb_engine_handler.cf_cluster(), &key)
            .unwrap();
        if value.is_none() {
            ConfState::default()
        } else {
            return ConfState::decode(value.unwrap().as_ref())
                .map_err(|e| tonic::Status::invalid_argument(e.to_string()))
                .unwrap();
        }
    }
}

impl RaftMachineStorage {
    pub fn uncommit_index(&self) -> HashMap<u64, i8> {
        let key = key_name_uncommit();
        match self
            .rocksdb_engine_handler
            .read::<Vec<u8>>(self.rocksdb_engine_handler.cf_cluster(), &key)
        {
            Ok(data) => {
                if let Some(value) = data {
                    match deserialize(value.as_ref()) {
                        Ok(v) => return v,
                        Err(err) => error!("{}", err),
                    }
                }
            }
            Err(err) => error!("{}", err),
        }
        HashMap::new()
    }
}

impl RaftMachineStorage {
    pub fn create_snapshot_metadata(&self) -> SnapshotMetadata {
        let hard_state = self.hard_state();
        let conf_state = self.conf_state();

        let mut meta: SnapshotMetadata = SnapshotMetadata::default();

        return meta;
    }
}
