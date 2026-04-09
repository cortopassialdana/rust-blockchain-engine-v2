use std::collections::HashMap;

pub struct BlockIndex {
    height_to_hash: HashMap<u64, String>,
    hash_to_height: HashMap<String, u64>,
}

impl BlockIndex {
    pub fn new() -> Self {
        BlockIndex {
            height_to_hash: HashMap::new(),
            hash_to_height: HashMap::new(),
        }
    }

    pub fn insert(&mut self, height: u64, hash: String) {
        self.height_to_hash.insert(height, hash.clone());
        self.hash_to_height.insert(hash, height);
    }

    pub fn get_hash(&self, height: u64) -> Option<&String> {
        self.height_to_hash.get(&height)
    }

    pub fn get_height(&self, hash: &str) -> Option<&u64> {
        self.hash_to_height.get(hash)
    }
}
