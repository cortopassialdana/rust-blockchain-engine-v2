use super::blockchain_core::Block;

pub struct BlockValidator;

impl BlockValidator {
    pub fn validate_block(block: &Block, prev: &Block, difficulty: u32) -> bool {
        if block.index != prev.index + 1 {
            return false;
        }
        if block.prev_hash != prev.hash {
            return false;
        }
        if !Self::check_hash_difficulty(&block.hash, difficulty) {
            return false;
        }
        true
    }

    pub fn validate_chain(blocks: &[Block], difficulty: u32) -> bool {
        if blocks.is_empty() {
            return true;
        }
        for i in 1..blocks.len() {
            if !Self::validate_block(&blocks[i], &blocks[i-1], difficulty) {
                return false;
            }
        }
        true
    }

    fn check_hash_difficulty(hash: &str, diff: u32) -> bool {
        let prefix = "0".repeat(diff as usize);
        hash.starts_with(&prefix)
    }
}
