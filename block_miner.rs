use super::blockchain_core::Block;

pub struct BlockMiner;

impl BlockMiner {
    pub fn mine_block(mut block: Block, difficulty: u32) -> Block {
        let target = "0".repeat(difficulty as usize);
        let mut nonce = 0u64;
        loop {
            block.nonce = nonce;
            let hash = Self::compute_block_hash(&block);
            if hash.starts_with(&target) {
                block.hash = hash;
                break;
            }
            nonce += 1;
        }
        block
    }

    fn compute_block_hash(block: &Block) -> String {
        let data = format!(
            "{}{}{}{}{}",
            block.index, block.timestamp, block.data, block.prev_hash, block.nonce
        );
        crate::crypto_hash::sha256_hash(data.as_bytes())
    }
}
