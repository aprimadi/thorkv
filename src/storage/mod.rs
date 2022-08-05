use std::collections::HashMap;

pub mod lfmap;

pub trait KeyValueStorage {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn put(&self, key: &[u8], value: &[u8]);
    fn delete(&self, key: &[u8]);
    fn keys(&self) -> Vec<Vec<u8>>;
}

pub struct StableStorage {
    stable_index: HashMap<Vec<u8>, usize>,
    stable_storage: Vec<StableStorageEntry>,
}

pub struct StableStorageEntry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}
