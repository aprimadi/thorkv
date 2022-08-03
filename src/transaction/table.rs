use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use skiplist::OrderedSkipList;

use crate::types::{CheckpointPhase, Xid};

pub type TransactionTableRef = Arc<TransactionTable>;

/// Keeps track of currently active transactions.
///
/// It generates a monotonically increasing transaction id (xid) during 
/// operation and stores active transaction entry in ascending order using 
/// skip list data structure.
pub struct TransactionTable {
    // Next xid
    next_xid: AtomicU64,
    phase: CheckpointPhase,
    // TODO: Maybe RwLock is better?????
    active_xids: Mutex<OrderedSkipList<Xid>>,
}

impl TransactionTable {
    // TODO
    pub fn new() -> Self {
        Self { 
            next_xid: AtomicU64::new(1),
            phase: CheckpointPhase::REST,
            active_xids: Mutex::new(OrderedSkipList::new()),
        }
    }
    
    /// Start a new transaction, get the new transaction id
    pub fn begin(&self) -> Xid {
        let xid = self.next_xid.fetch_add(1, Ordering::Relaxed);
        let mut active_xids = self.active_xids.lock().unwrap();
        active_xids.insert(xid);
        xid
    }
    
    /// Mark a transaction given by xid as completed.
    pub fn end(&self, xid: &Xid) {
        let mut active_xids = self.active_xids.lock().unwrap();
        active_xids.remove(xid);
    }
    
    pub fn next_xid(&self) -> Xid {
        let next_xid = self.next_xid.load(Ordering::Relaxed);
        next_xid
    }
    
    /// Returns the oldest transaction id that is still active.
    pub fn oldest_xid(&self) -> Option<u64> {
        let active_xids = self.active_xids.lock().unwrap();
        active_xids.front().map(|x| x.clone())
    }
    
    /// Move to the next checkpointing phase, return the first xid of the next
    /// phase
    pub fn move_to_next_phase(&self) -> u64 {
        // TODO
        1
    }
}
