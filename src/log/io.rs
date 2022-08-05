use std::fs::{File, OpenOptions};
use std::io::{Cursor, Read, Write};

use crate::log::logentry::LogEntry;
use crate::util::serde;
use crate::util::serde::Serialize;

// TODO: 
// [ ] Batched log writer
// [ ] Batched log reader

/// Encapsulates writing LogRecord to disk
/// 
/// Log format
///
///  ----------------------------------------------------------
/// | log_type_1 | log_data_1 | ... | log_type_N | log_data_N |
///  ----------------------------------------------------------
///
/// Explanation:
/// The write always write the type of the log struct before the other member
/// of the log struct itself. This way the reader know the boundary of the 
/// next log entry to read.
///
/// When serializing log record to file, whenever there is a choice between 
/// big-endian and little-endian, we always choose big-endian.
///
pub struct LogWriter {
    log_filepath: String,
    file: File,
}

impl LogWriter {
    pub fn new() -> Self {
        // TODO: Don't hardcode log path
        Self::with_path(String::from("wal.log"))
    }
    
    pub fn with_path(log_filepath: String) -> Self {
        let file = OpenOptions::new() 
            .append(true)
            .create(true)
            .open(&log_filepath)
            .unwrap();
        Self {
            log_filepath,
            file 
        }
    }
    
    pub fn write(&mut self, log: &LogEntry) -> std::io::Result<()> {
        let mut res = Vec::new();
        let buf = log.serialize();
        serde::serialize_u8_vec(&mut res, &buf);
        self.file.write_all(&res)?;
        Ok(())
    }
    
    pub fn flush(&self) {
        self.file.sync_data();
    }
}

const USIZE_LEN: usize = std::mem::size_of::<usize>();

/// Encapsulates reading LogRecord from disk
struct LogReader {
    log_filepath: String,
    file: File
}

impl LogReader {
    fn new() -> Self {
        // TODO: Don't hardcode log path
        Self::with_path(String::from("wal.log"))
    }
    
    fn with_path(log_filepath: String) -> Self {
        let file = File::open(&log_filepath).unwrap();
        Self {
            log_filepath,
            file,
        }
    }
    
    /// Read the next log record, returning None if EOF is reached
    fn read(&mut self) -> Option<LogEntry> {
        let mut size_buf: [u8; USIZE_LEN] = [0; USIZE_LEN];
        self.file.read_exact(&mut size_buf);
        let size: usize = serde::deserialize_usize(
            &mut Cursor::new(&size_buf)
        );
        
        let mut struct_buf = vec![0u8; size];
        self.file.read_exact(&mut struct_buf);
        let res = LogEntry::deserialize(&struct_buf);
        res
    }
}

fn serialize_log_record(log: &LogEntry) -> &[u8] {
    let bytes = unsafe { any_as_u8_slice(log) };
    bytes
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::log::logentry::LogicalOperation;
    
    #[test]
    fn write_and_read_log() {
        let logs = [
            LogEntry::Update {
                xid: 1,
                key: "foo".as_bytes().to_vec(),
                value: Some("bar".as_bytes().to_vec()),
                previous_value: None,
            },
        ];
        
        {
            let mut writer = LogWriter::new();
            writer.write(&logs[0]).unwrap();
            writer.flush();
        }
        
        let mut reader = LogReader::new();
        let log1 = reader.read().unwrap();
        assert_eq!(log1, logs[0]);
    }
}
