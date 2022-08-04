// Transaction ID types
pub type Xid = u64;

#[derive(Debug)]
pub struct Error {
    message: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckpointPhase {
    REST = 1,
    PREPARE,
    RESOLVE,
    CAPTURE,
    COMPLETE,
}
