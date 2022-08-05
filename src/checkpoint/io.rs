use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::util::serde;

const BATCH_SIZE: u64 = 512;

pub struct CheckpointWriter {
    file: File,
    written: u64,
}

impl CheckpointWriter {
    pub fn new() -> Self {
        Self::with_path("checkpoint.bin")
    }
    
    pub fn with_path(filepath: &str) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .open(filepath)
            .unwrap();
        Self {
            file,
            written: 0,
        }
    }
    
    pub fn append(&mut self, key: &[u8], value: &[u8]) {
        let mut res = vec![];
        serde::serialize_u8_vec(&mut res, key);
        serde::serialize_u8_vec(&mut res, value);
        self.file.write_all(&res).unwrap();
        
        self.written += 1;
        if self.written >= BATCH_SIZE {
            self.file.sync_data().unwrap();
            self.written = 0;
        }
    }
    
    pub fn flush(&self) {
        self.file.sync_data().unwrap();
    }
}
