use crate::kvstore::KVStore;

pub struct KVApp {
    store: KVStore,
    last_applied: usize,
}

impl KVApp {
    pub fn new() -> Self {
        KVApp {
            store: KVStore::new(),
            last_applied: 0,
        }
    }

    pub fn apply_command() {
        todo!();
    }

    pub fn update_store(&mut self, input: String) -> std::io::Result<String> {
        let store = &mut self.store;
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let response = match parts.as_slice() {
            ["get", key] if key.is_ascii() => store.get(*key).unwrap_or("Not found".to_string()),
            ["set", key, val] if key.is_ascii() && val.is_ascii() => match store.set(*key, *val) {
                Some(old) => format!("Old value: {}", old),
                None => "Set".to_string(),
            },
            ["delete", key] if key.is_ascii() => match store.delete(*key) {
                Some(old) => format!("Deleted value: {}", old),
                None => "No such key".to_string(),
            },
            ["incr", key] if key.is_ascii() => match store.get(*key) {
                Some(val) => match val.parse::<i32>() {
                    Ok(n) => {
                        let new_val = n + 1;
                        store.set(*key, &new_val.to_string());
                        format!("Incr {}", new_val)
                    }
                    Err(_) => "Invalid: can only incr integral values".to_string(),
                },
                None => "Invalid: no such key".to_string(),
            },
            _ => "Invalid command".to_string(),
        };
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_existing_key() {
        let mut kvapp = KVApp::new();
        kvapp.store.set("foo", "bar");
        let result = kvapp.update_store("get foo".to_string()).unwrap();
        assert_eq!(result, "bar");
    }

    #[test]
    fn test_get_nonexistent_key() {
        let mut kvapp = KVApp::new();
        let result = kvapp.update_store("get missing".to_string()).unwrap();
        assert_eq!(result, "Not found");
    }

    #[test]
    fn test_set_new_key() {
        let mut kvapp = KVApp::new();
        let result = kvapp.update_store("set mykey myvalue".to_string()).unwrap();
        assert_eq!(result, "Set");
        assert_eq!(kvapp.store.get("mykey"), Some("myvalue".to_string()));
    }

    #[test]
    fn test_set_existing_key() {
        let mut kvapp = KVApp::new();
        kvapp.store.set("key", "old");
        let result = kvapp.update_store("set key new".to_string()).unwrap();
        assert_eq!(result, "Old value: old");
        assert_eq!(kvapp.store.get("key"), Some("new".to_string()));
    }

    #[test]
    fn test_delete_existing_key() {
        let mut kvapp = KVApp::new();
        kvapp.store.set("foo", "bar");
        let result = kvapp.update_store("delete foo".to_string()).unwrap();
        assert_eq!(result, "Deleted value: bar");
        assert_eq!(kvapp.store.get("foo"), None);
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let mut kvapp = KVApp::new();
        let result = kvapp.update_store("delete missing".to_string()).unwrap();
        assert_eq!(result, "No such key");
    }

    #[test]
    fn test_incr_existing_integer() {
        let mut kvapp = KVApp::new();
        kvapp.store.set("counter", "5");
        let result = kvapp.update_store("incr counter".to_string()).unwrap();
        assert_eq!(result, "Incr 6");
        assert_eq!(kvapp.store.get("counter"), Some("6".to_string()));
    }

    #[test]
    fn test_incr_negative_integer() {
        let mut kvapp = KVApp::new();
        kvapp.store.set("counter", "-10");
        let result = kvapp.update_store("incr counter".to_string()).unwrap();
        assert_eq!(result, "Incr -9");
        assert_eq!(kvapp.store.get("counter"), Some("-9".to_string()));
    }

    #[test]
    fn test_incr_zero() {
        let mut kvapp = KVApp::new();
        kvapp.store.set("counter", "0");
        let result = kvapp.update_store("incr counter".to_string()).unwrap();
        assert_eq!(result, "Incr 1");
        assert_eq!(kvapp.store.get("counter"), Some("1".to_string()));
    }

    #[test]
    fn test_incr_noninteger_value() {
        let mut kvapp = KVApp::new();
        kvapp.store.set("key", "notanumber");
        let result = kvapp.update_store("incr key".to_string()).unwrap();
        assert_eq!(result, "Invalid: can only incr integral values");
    }

    #[test]
    fn test_incr_nonexistent_key() {
        let mut kvapp = KVApp::new();
        let result = kvapp.update_store("incr missing".to_string()).unwrap();
        assert_eq!(result, "Invalid: no such key");
    }

    #[test]
    fn test_invalid_command() {
        let mut kvapp = KVApp::new();
        let result = kvapp.update_store("invalid".to_string()).unwrap();
        assert_eq!(result, "Invalid command");
    }

    #[test]
    fn test_invalid_get_no_key() {
        let mut kvapp = KVApp::new();
        let result = kvapp.update_store("get".to_string()).unwrap();
        assert_eq!(result, "Invalid command");
    }

    #[test]
    fn test_invalid_set_missing_value() {
        let mut kvapp = KVApp::new();
        let result = kvapp.update_store("set key".to_string()).unwrap();
        assert_eq!(result, "Invalid command");
    }

    #[test]
    fn test_invalid_delete_no_key() {
        let mut kvapp = KVApp::new();
        let result = kvapp.update_store("delete".to_string()).unwrap();
        assert_eq!(result, "Invalid command");
    }

    #[test]
    fn test_extra_whitespace() {
        let mut kvapp = KVApp::new();
        let result = kvapp
            .update_store("  set   key   value  ".to_string())
            .unwrap();
        assert_eq!(result, "Set");
        assert_eq!(kvapp.store.get("key"), Some("value".to_string()));
    }

    #[test]
    fn test_multiple_incr() {
        let mut kvapp = KVApp::new();
        kvapp.store.set("count", "0");
        kvapp.update_store("incr count".to_string()).unwrap();
        kvapp.update_store("incr count".to_string()).unwrap();
        let result = kvapp.update_store("incr count".to_string()).unwrap();
        assert_eq!(result, "Incr 3");
        assert_eq!(kvapp.store.get("count"), Some("3".to_string()));
    }
}
