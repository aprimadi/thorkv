use std::convert::TryFrom;

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

impl TryFrom<u8> for CheckpointPhase {
    type Error = Error;
    
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::REST as u8     => Ok(Self::REST),
            x if x == Self::PREPARE as u8  => Ok(Self::PREPARE),
            x if x == Self::RESOLVE as u8  => Ok(Self::RESOLVE),
            x if x == Self::CAPTURE as u8  => Ok(Self::CAPTURE),
            x if x == Self::COMPLETE as u8 => Ok(Self::COMPLETE),
            _ => {
                Err(
                    Self::Error { 
                        message: format!("Unknown checkpoint phase: {}", v) 
                    }
                )
            },
        }
    }
}
