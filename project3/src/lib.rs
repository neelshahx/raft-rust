use std::collections::HashMap;

pub const SERVERS : [(u8, &str); 5] = [
    (0, "127.0.0.1:20000"),
    (1, "127.0.0.1:21000"),
    (2, "127.0.0.1:22000"),
    (3, "127.0.0.1:23000"),
    (4, "127.0.0.1:24000")
];

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_store_is_empty() {
        let store = KVStore::new();
        assert_eq!(store.get("any_key"), None);
    }

    #[test]
    fn test_set_and_get() {
        let mut store = KVStore::new();
        let result = store.set("key1", "value1");
        assert_eq!(result, None);
        assert_eq!(store.get("key1"), Some("value1".to_string()));
    }

    #[test]
    fn test_set_returns_old_value() {
        let mut store = KVStore::new();
        store.set("key1", "value1");
        let old_value = store.set("key1", "value2");
        assert_eq!(old_value, Some("value1".to_string()));
        assert_eq!(store.get("key1"), Some("value2".to_string()));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let store = KVStore::new();
        assert_eq!(store.get("nonexistent"), None);
    }

    #[test]
    fn test_delete_existing_key() {
        let mut store = KVStore::new();
        store.set("key1", "value1");
        let deleted = store.delete("key1");
        assert_eq!(deleted, Some("value1".to_string()));
        assert_eq!(store.get("key1"), None);
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let mut store = KVStore::new();
        let result = store.delete("nonexistent");
        assert_eq!(result, None);
    }

    #[test]
    fn test_multiple_keys() {
        let mut store = KVStore::new();
        store.set("key1", "value1");
        store.set("key2", "value2");
        store.set("key3", "value3");

        assert_eq!(store.get("key1"), Some("value1".to_string()));
        assert_eq!(store.get("key2"), Some("value2".to_string()));
        assert_eq!(store.get("key3"), Some("value3".to_string()));

        store.delete("key2");
        assert_eq!(store.get("key2"), None);
        assert_eq!(store.get("key1"), Some("value1".to_string()));
        assert_eq!(store.get("key3"), Some("value3".to_string()));
    }

    #[test]
    fn test_overwrite_multiple_times() {
        let mut store = KVStore::new();
        assert_eq!(store.set("key", "v1"), None);
        assert_eq!(store.set("key", "v2"), Some("v1".to_string()));
        assert_eq!(store.set("key", "v3"), Some("v2".to_string()));
        assert_eq!(store.get("key"), Some("v3".to_string()));
    }
}
