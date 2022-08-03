pub struct NaiveHashTable {
    
}

impl KeyValueStorage for NaiveHashTable {
    fn get<K>(key: K) -> Result<Option<Vec<u8>>, StorageError>
    where K: AsRef<[u8]>
    {
        // TODO
        Ok(None)
    }
    
    fn put<K, V>(key: K, value: V) -> Result<(), StorageError>
    where 
        K: AsRef<[u8]>,
        V: AsRef<[u8]>
    {
        // TODO
        Ok(())
    }
    
    fn delete<K>(key: K) -> Result<(), StorageError>
    where
        K: AsRef<[u8]> 
    {
        // TODO
        Ok(())
    }
}
