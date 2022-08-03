pub struct StorageError {
    message: String,
}

pub trait KeyValueStorage {
    fn get<K>(key: K) -> Result<Option<Vec<u8>>, StorageError>
    where K: AsRef<[u8]>;
        
    fn put<K, V>(key: K, value: V) -> Result<(), StorageError>
    where 
        K: AsRef<[u8]>,
        V: AsRef<[u8]>;
        
    fn delete<K>(key: K) -> Result<(), StorageError>
    where
        K: AsRef<[u8]>;
}
