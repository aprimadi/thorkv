/// DB is the main interface to ThorKV.
///
/// There are two storages, one is for live version and the other is for 
/// stable version. We expect the size of the stable version storage remains
/// small since it's content are removed when the record is written to disk.
///
/// We also keep track of a map from a "key" to whether there is a stable 
/// version for that key.

use std::sync::{Arc, RwLock};

use lockfree::set::Set;

use crate::checkpoint::{Checkpointer, start_checkpointer};
use crate::storage::KeyValueStorage;
use crate::storage::lfcuckoo::LFCuckooStorage;
use crate::transaction::table::{TransactionTable, TransactionTableRef};
use crate::types::{CheckpointPhase, Error, Xid};

pub type DBRef = Arc<DB>;

pub struct DB {
    xtable: TransactionTableRef,
    phase: RwLock<CheckpointPhase>,
    live_storage: Arc<dyn KeyValueStorage + Send + Sync>,
    stable_storage: Arc<dyn KeyValueStorage + Send + Sync>,
    // We need a graveyard to keep the set of keys that has been deleted
    // on the live version but still alive on the stable version.
    graveyard: Set<Vec<u8>>,
}

impl DB {
    pub fn open(path: &str) -> DBRef {
        // TODO: Start recovery
        
        let xtable = Arc::new(TransactionTable::new());
        
        let db = Arc::new(
            Self {
                xtable: xtable.clone(),
                phase: RwLock::new(CheckpointPhase::REST),
                live_storage: Arc::new(LFCuckooStorage::new()),
                stable_storage: Arc::new(LFCuckooStorage::new()),
                graveyard: Set::new(),
            }
        );
        
        // Start checkpointer
        let checkpointer = Arc::new(Checkpointer::new(db.clone(), xtable));
        start_checkpointer(checkpointer);
        
        db
    }
    
    pub fn get<K>(&self, key: K) -> Result<Option<Vec<u8>>, Error>
    where K: AsRef<[u8]>
    {
        self.live_storage.get(key.as_ref())
    }
        
    pub fn put<K, V>(&self, key: K, value: V) -> Result<(), Error>
    where 
        K: AsRef<[u8]>,
        V: AsRef<[u8]>
    {
        // TODO
        Ok(())
    }
        
    pub fn delete<K>(&self, key: K) -> Result<(), Error>
    where
        K: AsRef<[u8]>
    {
        Ok(())
    }
    
    pub fn set_phase(&self, phase: CheckpointPhase) -> Xid {
        // TODO: There can be a race condition between the time we fetch xid 
        // until the time we set phase. Could be a problem???????
        let mut phase_guard = self.phase.write().unwrap();
        *phase_guard = phase;
        let xid = self.xtable.next_xid();
        xid
    }
    
    pub fn current_phase(&self) -> CheckpointPhase {
        let phase = self.phase.read().unwrap();
        *phase
    }
    
    pub fn save_checkpoint(&self) {
        // TODO: This should iterate over all key values and save it to disk
        // Setting the stable_status and cleaning up stable record along the
        // way.
    }
    
    pub fn post_checkpoint(&self) {
        // TODO: This should clean up stable_status bit
    }
}
