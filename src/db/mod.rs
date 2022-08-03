/// DB is the main interface to ThorKV.
///
/// There are two storages, one is for live version and the other is for 
/// stable version. We expect the size of the stable version storage remains
/// small since it's content are removed when the record is written to disk.
///
/// We also keep track of a map from a "key" to whether there is a stable 
/// version for that key.

use std::sync::Arc;

use crate::Error;
use crate::storage::KeyValueStorage;

pub type DBRef = Arc<DB>;

pub struct DB {
    live_storage: Arc<dyn KeyValueStorage + Send + Sync>,
    stable_storage: Arc<dyn KeyValueStorage + Send + Sync>,
}

impl DB {
    pub fn new(
        live_storage: Arc<dyn KeyValueStorage + Send + Sync>, 
        stable_storage: Arc<dyn KeyValueStorage + Send + Sync>,
    ) -> Self {
        Self {
            live_storage,
            stable_storage,
        }
    }
    
    pub fn get<K>(key: K) -> Result<Option<Vec<u8>>, Error>
    where K: AsRef<[u8]>
    {
        // TODO
        Ok(None)
    }
        
    pub fn put<K, V>(key: K, value: V) -> Result<(), Error>
    where 
        K: AsRef<[u8]>,
        V: AsRef<[u8]>
    {
        // TODO
        Ok(())
    }
        
    pub fn delete<K>(key: K) -> Result<(), Error>
    where
        K: AsRef<[u8]>
    {
        Ok(())
    }
}
