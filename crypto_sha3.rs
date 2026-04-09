use sha3::{Sha3_256, Digest};

pub struct Sha3Hash;

impl Sha3Hash {
    pub fn hash(data: &[u8]) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn hash_string(s: &str) -> String {
        Self::hash(s.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sha3() {
        let h = Sha3Hash::hash_string("test");
        assert_eq!(h.len(), 64);
    }
}
