use super::blockchain_core::Block;
use std::collections::HashMap;

pub struct BlockStorage {
    memory: HashMap<u64, Block>,
    path: String,
}

impl BlockStorage {
    pub fn new(path: String) -> Self {
        BlockStorage {
            memory: HashMap::new(),
            path,
        }
    }

    pub fn save_block(&mut self, block: Block) {
        self.memory.insert(block.index, block.clone());
        self.sync_to_disk(&block);
    }

    fn sync_to_disk(&self, block: &Block) {
        let filename = format!("{}/block_{}.dat", self.path, block.index);
        println!("Saved block {} to disk: {}", block.index, filename);
    }

    pub fn load_block(&self, index: u64) -> Option<Block> {
        self.memory.get(&index).cloned()
    }

    pub fn latest_block(&self) -> Option<Block> {
        self.memory.values().max_by_key(|b| b.index).cloned()
    }
}
