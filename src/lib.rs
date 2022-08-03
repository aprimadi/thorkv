mod checkpoint;
mod constants;
mod db;
mod log;
mod storage;
mod transaction;
mod types;

pub struct Error {
    message: String,
}
