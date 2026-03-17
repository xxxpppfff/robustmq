use common_base::errors::RobustMQError;

use super::StorageDataWrap;
use crate::storage::rocksdb::{RocksDBEngine, DB_COLUMN_FAMILY_CLUSTER};
use serde::Serialize;
use std::sync::Arc;

// TODO: 保存数据
pub fn engine_save_by_cluster<T>(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    key_name: String,
    value: T,
) -> Result<(), RobustMQError>
where
    T: Serialize,
{
    return engine_save(
        rocksdb_engine_handler,
        DB_COLUMN_FAMILY_CLUSTER,
        key_name,
        value,
    );
}

// TODO: 获取数据
pub fn engine_get_by_cluster(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    key_name: String,
) -> Result<Option<StorageDataWrap>, RobustMQError> {
    return engine_get(rocksdb_engine_handler, DB_COLUMN_FAMILY_CLUSTER, key_name);
}

// TODO: 数据是否存在
pub fn engine_exists_by_cluster(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    key_name: String,
) -> Result<bool, RobustMQError> {
    return engine_exists(rocksdb_engine_handler, DB_COLUMN_FAMILY_CLUSTER, key_name);
}

// TODO: 删除数据
pub fn engine_delete_by_cluster(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    key_name: String,
) -> Result<(), RobustMQError> {
    return engine_delete(rocksdb_engine_handler, DB_COLUMN_FAMILY_CLUSTER, key_name);
}

// TODO: 获取前缀数据
pub fn engine_prefix_list_by_cluster(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    key_name: String,
) -> Result<Vec<StorageDataWrap>, RobustMQError> {
    return engine_prefix_list(rocksdb_engine_handler, DB_COLUMN_FAMILY_CLUSTER, key_name);
}

fn engine_save<T>(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    rocksdb_cluster: &str,
    key_name: String,
    value: T,
) -> Result<(), RobustMQError>
where
    T: Serialize,
{
    let cf = if rocksdb_cluster.to_string() == DB_COLUMN_FAMILY_CLUSTER.to_string() {
        rocksdb_engine_handler.cf_cluster()
    } else {
        return Err(RobustMQError::ClusterNoAvailableNode);
    };

    let content = match serde_json::to_vec(&value) {
        Ok(data) => data,
        Err(e) => return Err(RobustMQError::CommonError(e.to_string())),
    };

    let data = StorageDataWrap::new(content);
    match rocksdb_engine_handler.write(cf, &key_name, &data) {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(RobustMQError::CommonError(e.to_string()));
        }
    }
}

fn engine_get(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    rocksdb_cluster: &str,
    key_name: String,
) -> Result<Option<StorageDataWrap>, RobustMQError> {
    let cf = if rocksdb_cluster.to_string() == DB_COLUMN_FAMILY_CLUSTER.to_string() {
        rocksdb_engine_handler.cf_cluster()
    } else {
        return Err(RobustMQError::ClusterNoAvailableNode);
    };
    match rocksdb_engine_handler.read::<StorageDataWrap>(cf, &key_name) {
        Ok(Some(data)) => {
            return Ok(Some(data));
        }
        Ok(None) => {
            return Ok(None);
        }
        Err(e) => return Err(RobustMQError::CommonError(e))
        
    }
}

fn engine_exists(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    rocksdb_cluster: &str,
    key_name: String,
) -> Result<bool, RobustMQError> {
    let cf = if rocksdb_cluster.to_string() == DB_COLUMN_FAMILY_CLUSTER.to_string() {
        rocksdb_engine_handler.cf_cluster()
    } else {
        return Err(RobustMQError::ClusterNoAvailableNode);
    };

    return Ok(rocksdb_engine_handler.exists(cf, &key_name));
}

fn engine_delete(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    rocksdb_cluster: &str,
    key_name: String,
) -> Result<(), RobustMQError> {
    let cf = if rocksdb_cluster.to_string() == DB_COLUMN_FAMILY_CLUSTER.to_string() {
        rocksdb_engine_handler.cf_cluster()
    } else {
        return Err(RobustMQError::ClusterNoAvailableNode);
    };
    rocksdb_engine_handler.delete(cf, &key_name)
}

fn engine_prefix_list(
    rocksdb_engine_handler: Arc<RocksDBEngine>,
    rocksdb_cluster: &str,
    prefix_key_name: String,
) -> Result<Vec<StorageDataWrap>, RobustMQError> {
    let cf = if rocksdb_cluster.to_string() == DB_COLUMN_FAMILY_CLUSTER.to_string() {
        rocksdb_engine_handler.cf_cluster()
    } else {
        return Err(RobustMQError::ClusterNoAvailableNode);
    };
    let data_list = rocksdb_engine_handler.read_prefix(cf, &prefix_key_name);
    let mut result = Vec::new();
    for raw in data_list {
        for (_, v) in raw {
            match serde_json::from_slice::<StorageDataWrap>(v.as_ref()) {
                Ok(v) => result.push(v),
                Err(_) => continue
            }
        }
    }
    return Ok(result);
}