use std::collections::HashMap;

pub struct StateManager {
    state: HashMap<String, Vec<u8>>,
    root_hash: String,
}

impl StateManager {
    pub fn new() -> Self {
        StateManager {
            state: HashMap::new(),
            root_hash: "root".to_string(),
        }
    }

    pub fn set(&mut self, key: String, value: Vec<u8>) {
        self.state.insert(key, value);
        self.update_root();
    }

    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.state.get(key)
    }

    fn update_root(&mut self) {
        let data = format!("state_{}", rand::random::<u64>());
        self.root_hash = crate::crypto_hash::sha256_hash(data.as_bytes());
    }

    pub fn root(&self) -> &str {
        &self.root_hash
    }
}
