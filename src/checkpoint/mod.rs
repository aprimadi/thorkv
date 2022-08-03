use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread;

use crate::constants::CHECKPOINT_INTERVAL_SECS;
use crate::db::DBRef;
use crate::transaction::table::TransactionTableRef;
use crate::types::{CheckpointPhase, Xid};

const BUSY_WAIT_INTERVAL_MILLIS: u64 = 1;

// TODO: Checkpointer and DB is tightly coupled, figure out a way to decouple 
// it.
pub struct Checkpointer {
    // TODO: The phase needs to be protected by mutex since it's accessed by
    // DB to find out the current phase.
    //
    // TODO: I think the phase information should be part of transaction table
    // that way the current phase and the next_xid information can be updated
    // atomically.
    phase: RwLock<CheckpointPhase>,
    xtable: TransactionTableRef,
    db: Option<DBRef>,
}

impl Checkpointer {
    pub fn new(xtable: TransactionTableRef) -> Self {
        Self { phase: RwLock::new(CheckpointPhase::REST), xtable, db: None }
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
}

/// Start periodic checkpointer running on background thread
pub fn start_checkpointer(checkpointer: Arc<Checkpointer>) {
    tokio::spawn(async move {
        thread::sleep(Duration::from_secs(CHECKPOINT_INTERVAL_SECS));
        
        prepare_phase(checkpointer.clone()).await;
        resolve_phase(checkpointer.clone()).await;
        capture_phase().await;
        complete_phase().await;
    });
}

async fn prepare_phase(
    checkpointer: Arc<Checkpointer>,
) {
    let xtable = checkpointer.xtable.clone();
    
    // Transition to prepare phase
    let prepare_xid = checkpointer.set_phase(CheckpointPhase::PREPARE);
    
    // Wait until all transaction that started in rest phase already 
    // completed.
    loop {
        let oldest_xid = xtable.oldest_xid();
        if oldest_xid.is_some() && oldest_xid.unwrap() >= prepare_xid {
            break;
        }
        thread::sleep(Duration::from_millis(BUSY_WAIT_INTERVAL_MILLIS));
    }
}

async fn resolve_phase(checkpointer: Arc<Checkpointer>) {
    let xtable = checkpointer.xtable.clone();
    
    // Transition to resolve phase
    let resolve_xid = checkpointer.set_phase(CheckpointPhase::RESOLVE);
    
    // Wait until all transactions that started during prepare phase have
    // completed.
    loop {
        let oldest_xid = xtable.oldest_xid();
        if oldest_xid.is_some() && oldest_xid.unwrap() >= resolve_xid {
            break;
        }
        thread::sleep(Duration::from_millis(BUSY_WAIT_INTERVAL_MILLIS));
    }
}

// Iterate through all key values and store live/stable record to disk.
//
// This removes stable version and also set stable_status bit to available 
// during scan. The reason we set stable_status bit to available is so that no
// transaction during capture phase can set a stable version that is already
// removed.
async fn capture_phase() {
    // TODO
}

async fn complete_phase() {
    
}
