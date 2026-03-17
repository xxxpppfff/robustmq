use common_base::config::placement_center::PlacementCenterConfig;
use common_base::errors::RobustMQError;
use rocksdb::{ColumnFamily, DBCompactionStyle, Options, SliceTransform, DB};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

pub const DB_COLUMN_FAMILY_CLUSTER: &str = "cluster";

fn column_family_list() -> Vec<String> {
    let mut list = Vec::new();
    list.push(DB_COLUMN_FAMILY_CLUSTER.to_string());
    list
}

pub struct RocksDBEngine {
    pub db: DB,
}

impl RocksDBEngine {
    // 创建一个Rocksdb实例
    pub fn new(config: &PlacementCenterConfig) -> Self {
        let ops: Options = Self::open_db_opts();
        let db_path = format!("{}/{}", config.data_path, "_storage_rocksdb");
        if !Path::new(&db_path).exists() {
            // unwrap用于提取成功值，如果遇到错误则直接崩溃程序。
            DB::open(&ops, db_path.clone()).unwrap();
        }
        // 初始化列族
        let cf_list = rocksdb::DB::list_cf(&ops, &db_path).unwrap();
        let mut instance = DB::open_cf(&ops, db_path.clone(), &cf_list).unwrap();

        for family in column_family_list().iter() {
            // 如果列族不存在则创建
            if cf_list.iter().find(|cf| cf == &family).is_none() {
                // TODO:创建列族
                match instance.create_cf(family, &ops) {
                    Ok(_) => {}
                    Err(err) => {
                        panic!("Failed to create ColumnFamily: {:?}", err);
                    }
                }
            }
        }

        RocksDBEngine { db: instance }
    }

    // TODO: 写入数据
    // 流程：先选择ColumnFamily，通过serde_json序列化数据，最后通过put_cf将数据写入到RocksDB中
    pub fn write<T: Serialize + std::fmt::Debug>(
        &self,
        cf: &ColumnFamily,
        key: &str,
        value: &T,
    ) -> Result<(), String> {
        match serde_json::to_string(&value) {
            Ok(serialized) => self
                .db
                .put_cf(cf, key, serialized.into_bytes())
                .map_err(|e| format!("Failed to put to ColumnFamily:{:?}", e)),
            Err(err) => Err(format!(
                "Failed to serialize value to String. T: {:?}, err: {:?}",
                value, err
            )),
        }
    }

    // TODO: 写入数据
    pub fn read<T: DeserializeOwned>(
        &self,
        cf: &ColumnFamily,
        key: &str,
    ) -> Result<Option<T>, String> {
        match self.db.get_cf(cf, key) {
            Ok(opt) => match opt {
                None => Ok(None),
                Some(found) => match String::from_utf8(found) {
                    Ok(s) => match serde_json::from_str::<T>(&s) {
                        Ok(t) => Ok(Some(t)),
                        Err(err) => {
                            Err(format!("Failed to deserialize value to T. err: {:?}", err))
                        }
                    },
                    Err(err) => Err(format!("Failed to deserialize: {:?}", err)),
                },
            },
            Err(err) => Err(format!("Failed to get from ColumnFamily: {:?}", err)),
        }
    }

    // TODO: 根据前缀读取数据
    // 先通过seek方法找到前缀对应的第一个Key，再通过next方法一个一个往后获取数据，从而得到该前缀对应的所有Key
    pub fn read_prefix(
        &self,
        cf: &ColumnFamily,
        search_key: &str,
    ) -> Vec<HashMap<String, Vec<u8>>> {
        let mut iter = self.db.raw_iterator_cf(cf);
        iter.seek(search_key);

        let mut result = Vec::new();
        while iter.valid() {
            let key = iter.key();
            let value = iter.value();

            let mut raw = HashMap::new();

            if key == None || value == None {
                continue;
            }
            let result_key = match String::from_utf8(key.unwrap().to_vec()) {
                Ok(s) => s,
                Err(_) => continue,
            };
            // 判断获取到的数据的 Key 是否是搜索的前缀，否则，退出循环。
            if !result_key.starts_with(search_key) {
                break;
            }
            raw.insert(result_key, value.unwrap().to_vec());
            result.push(raw);

            iter.next();
        }
        result
    }

    // TODO: 根据key删除数据
    pub fn delete(&self, cf: &ColumnFamily, key: &str) -> Result<(), RobustMQError> {
        Ok(self.db.delete_cf(cf, key)?)
    }

    // TODO: 判断key是否存在
    pub fn exists(&self, cf: &ColumnFamily, key: &str) -> bool {
        self.db.key_may_exist_cf(cf, key)
    }

    // TODO: 配置初始化
    fn open_db_opts() -> Options {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_max_open_files(1000);
        opts.set_use_fsync(false);
        opts.set_bytes_per_sync(8388608);
        opts.optimize_for_point_lookup(1024);
        opts.set_table_cache_num_shard_bits(6);
        opts.set_max_write_buffer_number(32);
        opts.set_write_buffer_size(536870912);
        opts.set_target_file_size_base(1073741824);
        opts.set_min_write_buffer_number_to_merge(4);
        opts.set_level_zero_stop_writes_trigger(2000);
        opts.set_level_zero_slowdown_writes_trigger(0);
        opts.set_compaction_style(DBCompactionStyle::Universal);
        opts.set_disable_auto_compactions(true);

        let transform = SliceTransform::create_fixed_prefix(10);
        opts.set_prefix_extractor(transform);
        opts.set_memtable_prefix_bloom_ratio(0.2);

        opts
    }
}
