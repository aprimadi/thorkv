use std::collections::VecDeque;

use crate::log::io::LogWriter;
use crate::log::logentry::LogEntry;

mod io;
mod logentry;
mod serde;

const BATCH_SIZE: u32 = 32;

struct LogManager {
    writer: LogWriter,
    log_queue: VecDeque<LogEntry>,
}

impl LogManager {
    pub fn new() -> Self {
        Self {
            writer: LogWriter::new(),
            log_queue: VecDeque::new(),
        }
    }
    
    /// Start a thread that process redo record in batches and flush to disk
    pub async fn start(&self) {
        tokio::spawn(async {
            // TODO: Currently this write each log record in sync which isn't 
            // efficient. Perhaps try to batch log records?
            
            // First, copy and clear log_queue
        });
    }
    
    pub fn append_log(&mut self, log: LogEntry) {
        self.writer.write(&log).unwrap();
        self.writer.flush();
        //self.log_queue.push_back(log);
    }
}
