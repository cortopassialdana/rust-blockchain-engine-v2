pub struct MerkleTree {
    root: String,
    leaves: Vec<String>,
}

impl MerkleTree {
    pub fn new(leaves: Vec<String>) -> Self {
        let root = Self::build_root(&leaves);
        MerkleTree { root, leaves }
    }

    fn build_root(leaves: &[String]) -> String {
        if leaves.is_empty() {
            return "empty".to_string();
        }
        let mut current = leaves.to_vec();
        while current.len() > 1 {
            let mut next = Vec::new();
            for i in (0..current.len()).step_by(2) {
                let left = &current[i];
                let right = if i + 1 < current.len() { &current[i+1] } else { left };
                let combined = format!("{}{}", left, right);
                next.push(crate::crypto_hash::sha256_hash(combined.as_bytes()));
            }
            current = next;
        }
        current[0].clone()
    }

    pub fn root(&self) -> &str {
        &self.root
    }
}
