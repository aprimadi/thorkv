use lockfree::map::Map;

use crate::storage::KeyValueStorage;

pub struct LFMapStorage {
    map: Map<Vec<u8>, Vec<u8>>,
}

impl LFMapStorage {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl KeyValueStorage for LFMapStorage {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let v = self.map.get(key);
        let value = v.map(|x| x.clone().1);
        value
    }
    
    fn put(&self, key: &[u8], value: &[u8])  {
        self.map.insert(key.to_vec(), value.to_vec());
    }
    
    fn delete(&self, key: &[u8]) {
        self.map.remove(key);
    }

    fn keys(&self) -> Vec<Vec<u8>> {
        let mut keys = vec![];
        let mut iter = self.map.iter();
        loop {
            let item = iter.next();
            if item.is_none() {
                break;
            }
            let item = item.unwrap();
            keys.push(item.1.clone());
        }
        keys
    }
}
