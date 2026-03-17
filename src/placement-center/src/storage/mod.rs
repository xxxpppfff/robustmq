use common_base::tools::now_second;
use serde::{Deserialize, Serialize};

pub mod rocksdb;
pub mod kv;
pub mod engine;

#[derive(Serialize, Deserialize, Debug)]
pub struct StorageDataWrap {
    pub data: Vec<u8>,
    pub create_time: u64,
}

impl StorageDataWrap {
    pub fn new(data: Vec<u8>) -> Self {
        return StorageDataWrap { data, create_time: now_second()}
    }
}