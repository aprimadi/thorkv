use crate::types::Error;

pub mod lfcuckoo;

pub trait KeyValueStorage {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Error>;
    fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error>;
    fn delete(&self, key: &[u8]) -> Result<(), Error>;
}
