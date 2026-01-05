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
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    pub fn set(&mut self, key: &str, val: &str) {
        self.data.insert(key.to_string(), val.to_string());
    }
    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
    }
}
