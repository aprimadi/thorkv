use crate::Error;

mod naivehash;

pub trait KeyValueStorage {
    fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, Error>;
    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error>;
    fn delete(&mut self, key: &[u8]) -> Result<(), Error>;
}
