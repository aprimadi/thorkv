use std::convert::{TryFrom, TryInto};
use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::types::{CheckpointPhase, Xid};
use crate::util::serde;
use crate::util::serde::Serialize;

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
    Update { 
        xid: Xid, 
        key: Vec<u8>, 
        value: Option<Vec<u8>>, 
        previous_value: Option<Vec<u8>>,
    },
    CPhase(CheckpointPhase),
}

impl LogEntry {
    // TODO: This should return none if it cannot read the next record size
    pub fn deserialize(bytes: &[u8]) -> Option<LogEntry> {
        let mut rdr = Cursor::new(bytes);
        let lr_type: u8;
        match rdr.read_u8() {
            Ok(t) => lr_type = t,
            _     => return None,
        }
        
        let lr_type: LogEntryType = lr_type.try_into().unwrap();
        let mut log = None;
        match lr_type {
            LogEntryType::XBEGIN    => {
                let xid = serde::deserialize_xid(&mut rdr);
                log = Some(LogEntry::XBegin { xid });
            }
            LogEntryType::XCOMMIT   => {
                let xid = serde::deserialize_xid(&mut rdr);
                log = Some(LogEntry::XCommit { xid });
            }
            LogEntryType::XABORT    => {
                let xid = serde::deserialize_xid(&mut rdr);
                log = Some(LogEntry::XAbort { xid });
            }
            LogEntryType::UPDATE    => {
                let xid = serde::deserialize_xid(&mut rdr);
                let key = serde::deserialize_u8_vec(&mut rdr);
                let value;
                if rdr.read_u8().unwrap() == 1 {
                    value = Some(serde::deserialize_u8_vec(&mut rdr));
                } else {
                    value = None;
                }
                let previous_value;
                if rdr.read_u8().unwrap() == 1 {
                    previous_value = Some(serde::deserialize_u8_vec(&mut rdr));
                } else {
                    previous_value = None;
                }
                log = Some(LogEntry::Update { xid, key, value, previous_value });
            },
            LogEntryType::CPHASE    => {
                let phase_u8 = rdr.read_u8().unwrap();
                let phase = CheckpointPhase::try_from(phase_u8).unwrap();
                log = Some(LogEntry::CPhase(phase));
            }
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
            Self::Update { xid, key, value, previous_value } => {
                res.write_u8(LogEntryType::UPDATE as u8).unwrap();
                serde::serialize_xid(&mut res, xid);
                serde::serialize_u8_vec(&mut res, key);
                if value.is_some() {
                    res.write_u8(1).unwrap();
                    serde::serialize_u8_vec(&mut res, value.as_ref().unwrap());
                } else {
                    res.write_u8(0).unwrap();
                }
                if previous_value.is_some() {
                    res.write_u8(1).unwrap();
                    serde::serialize_u8_vec(
                        &mut res, 
                        previous_value.as_ref().unwrap()
                    );
                } else {
                    res.write_u8(0).unwrap();
                }
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
