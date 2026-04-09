use super::blockchain_core::Block;

pub struct GenesisBuilder {
    timestamp: u128,
    data: String,
    difficulty: u32,
}

impl GenesisBuilder {
    pub fn new() -> Self {
        GenesisBuilder {
            timestamp: 1704067200000,
            data: "Genesis Block".to_string(),
            difficulty: 2,
        }
    }

    pub fn with_timestamp(mut self, ts: u128) -> Self {
        self.timestamp = ts;
        self
    }

    pub fn with_data(mut self, data: &str) -> Self {
        self.data = data.to_string();
        self
    }

    pub fn build(self) -> Block {
        Block {
            index: 0,
            timestamp: self.timestamp,
            data: self.data,
            prev_hash: "0".to_string(),
            hash: crate::crypto_hash::sha256_hash(b"genesis"),
            nonce: 0,
        }
    }
}
