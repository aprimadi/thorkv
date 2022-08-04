use lockfree_cuckoohash::{pin, LockFreeCuckooHash};

use crate::storage::KeyValueStorage;
use crate::types::Error;

/// Lock-Free Cuckoo Hash Table based storage implementation.
pub struct LFCuckooStorage {
    map: LockFreeCuckooHash<Vec<u8>, Vec<u8>>,
}

impl LFCuckooStorage {
    pub fn new() -> Self {
        Self {
            map: LockFreeCuckooHash::new(),
        }
    }
}

impl KeyValueStorage for LFCuckooStorage {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        let guard = pin();
        let v = self.map.get(key, &guard);
        let value = v.map(|x| x.clone());
        Ok(value)
    }
    
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.map.insert(key.to_vec(), value.to_vec());
        Ok(())
    }
    
    fn delete(&self, key: &[u8]) -> Result<(), Error> {
        self.map.remove(key);
        Ok(())
    }
}
