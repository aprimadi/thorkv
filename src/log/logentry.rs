use std::convert::{TryFrom, TryInto};
use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::log::serde;
use crate::log::serde::Serialize;
use crate::types::Xid;

enum LogEntryType {
    XBEGIN = 1,
    XCOMMIT,
    XABORT,
    UPDATE,
    // Tracks the current checkpointing phase
    CPHASE,
}

impl TryFrom<u8> for LogEntryType {
    type Error = ();
    
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::XBEGIN as u8  => Ok(Self::XBEGIN),
            x if x == Self::XCOMMIT as u8 => Ok(Self::XCOMMIT),
            x if x == Self::XABORT as u8  => Ok(Self::XABORT),
            x if x == Self::UPDATE as u8  => Ok(Self::UPDATE),
            x if x == Self::CPHASE as u8  => Ok(Self::CPHASE),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LogEntry {
    XBegin { xid: Xid },
    XCommit { xid: Xid },
    XAbort { xid: Xid },
    Update { xid: Xid, operation: LogicalOperation },
    CPhase(CheckpointPhase),
}

impl LogEntry {
    // TODO: This should return none if it cannot read the next record size
    pub fn deserialize(bytes: &[u8]) -> Option<LogEntry> {
        let mut rdr = Cursor::new(bytes);
        let lr_type: u8;
        match rdr.read_u8() {
            Ok(t) => {
                lr_type = t;
            }
            _ => {
                return None;
            }
        }
        
        let lr_type: LogEntryType = lr_type.try_into().unwrap();
        let mut log = None;
        match lr_type {
            LogEntryType::XBEGIN    => {
                log = Some(LogEntry::XBegin { xid: 1 });
            }
            LogEntryType::XCOMMIT   => {
                log = Some(LogEntry::XCommit { xid: 1 });
            }
            LogEntryType::XABORT    => {
                log = Some(LogEntry::XAbort { xid: 1 });
            }
            LogEntryType::UPDATE    => {
                log = Some(LogEntry::Update {
                    xid: 1,
                    operation: LogicalOperation::Set { 
                        key: "foo".to_owned(), 
                        value: "bar".to_owned() 
                    },
                });
            },
            LogEntryType::CPHASE    => {
                log = Some(LogEntry::CPhase(CheckpointPhase::REST));
            }
            /*
            LogRecordType::Abort => {
                let mut abort = AbortRecord::empty();
                abort.deserialize(&mut rdr);
                LogRecord::Abort(abort)
            },
            */
        }
        log
    }
}

impl Serialize for LogEntry {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        match &self {
            Self::XBegin { xid } => {
                res.write_u8(LogEntryType::XBEGIN as u8).unwrap();
                serde::serialize_xid(&mut res, xid);
            },
            Self::XCommit { xid } => {
                res.write_u8(LogEntryType::XCOMMIT as u8).unwrap();
                serde::serialize_xid(&mut res, xid);
            },
            Self::XAbort { xid } => {
                res.write_u8(LogEntryType::XABORT as u8).unwrap();
                serde::serialize_xid(&mut res, xid);
            },
            Self::Update { xid, operation } => {
                res.write_u8(LogEntryType::UPDATE as u8).unwrap();
                serde::serialize_xid(&mut res, xid);
            }
            Self::CPhase(phase) => {
                res.write_u8(LogEntryType::CPHASE as u8).unwrap();
                res.write_u8(*phase as u8).unwrap();
            }
        }
        res
    }
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperation {
    Set { key: String, value: String },
    Delete { key: String },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CheckpointPhase {
    REST = 1,
    PREPARE,
    RESOLVE,
    CAPTURE,
    COMPLETE,
}
