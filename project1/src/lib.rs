use std::collections::HashMap;

pub struct KVStore {
    data: HashMap<String, String>,
}

impl KVStore {
    pub fn new() -> Self {
        KVStore {
            data: HashMap::new(),
        }
    }
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }
    pub fn set(&mut self, key: &str, val: &str) -> Option<String> {
        self.data.insert(key.to_string(), val.to_string())
    }
    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}
