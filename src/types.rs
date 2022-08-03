// Transaction ID types
pub type Xid = u64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckpointPhase {
    REST = 1,
    PREPARE,
    RESOLVE,
    CAPTURE,
    COMPLETE,
}
