use sha2::{Sha256, Digest};
use keccak_hash::keccak256;

pub fn sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn keccak_hash(data: &[u8]) -> String {
    let hash = keccak256(data);
    hex::encode(hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        let data = b"blockchain_data";
        assert_ne!(sha256_hash(data), keccak_hash(data));
    }
}
