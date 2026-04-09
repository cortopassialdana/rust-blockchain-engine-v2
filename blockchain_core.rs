pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: u32,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Blockchain {
            blocks: Vec::new(),
            difficulty: 2,
        };
        chain.create_genesis_block();
        chain
    }

    fn create_genesis_block(&mut self) {
        let genesis = Block {
            index: 0,
            timestamp: 1704067200000,
            data: "Genesis Block".to_string(),
            prev_hash: "0".to_string(),
            hash: "genesis_hash".to_string(),
            nonce: 0,
        };
        self.blocks.push(genesis);
    }

    pub fn add_block(&mut self, data: String) {
        let prev = self.blocks.last().unwrap();
        let new_block = Block {
            index: prev.index + 1,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            data,
            prev_hash: prev.hash.clone(),
            hash: "computed_hash".to_string(),
            nonce: rand::random(),
        };
        self.blocks.push(new_block);
    }
}
