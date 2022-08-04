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
    db: DBRef,
    xtable: TransactionTableRef,
}

impl Checkpointer {
    pub fn new(db: DBRef, xtable: TransactionTableRef) -> Self {
        Self { db, xtable }
    }
}

/// Start periodic checkpointer running on background thread
pub fn start_checkpointer(checkpointer: Arc<Checkpointer>) {
    tokio::spawn(async move {
        thread::sleep(Duration::from_secs(CHECKPOINT_INTERVAL_SECS));
        run_checkpointer(checkpointer);        
    });
}

fn run_checkpointer(checkpointer: Arc<Checkpointer>) {
    let db = checkpointer.db.clone();
    let xtable = checkpointer.xtable.clone();
    
    let prepare_xid = db.set_phase(CheckpointPhase::PREPARE);
    wait_oldest_xid_gte(xtable.clone(), prepare_xid);
    let resolve_xid = db.set_phase(CheckpointPhase::RESOLVE);
    wait_oldest_xid_gte(xtable.clone(), resolve_xid);
    db.set_phase(CheckpointPhase::CAPTURE);
    db.save_checkpoint();
    let complete_xid = db.set_phase(CheckpointPhase::COMPLETE);
    wait_oldest_xid_gte(xtable.clone(), complete_xid);
    db.post_checkpoint();
    db.set_phase(CheckpointPhase::REST);
}

// Busy wait until oldest xid >= xid
fn wait_oldest_xid_gte(xtable: TransactionTableRef, xid: Xid) {
    loop {
        let oldest_xid = xtable.oldest_xid();
        if oldest_xid.is_some() && oldest_xid.unwrap() >= xid {
            break;
        }
        thread::sleep(Duration::from_millis(BUSY_WAIT_INTERVAL_MILLIS));
    }
}
