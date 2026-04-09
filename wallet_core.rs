pub struct Wallet {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let (sk, pk) = crate::digital_signature::SignatureManager::generate_keys();
        let addr = Self::public_key_to_address(&pk);
        Wallet {
            private_key: sk.to_bytes().to_vec(),
            public_key: pk.to_bytes().to_vec(),
            address: addr,
        }
    }

    fn public_key_to_address(pk: &ed25519_dalek::VerifyingKey) -> String {
        let hash = crate::crypto_hash::keccak_hash(&pk.to_bytes());
        format!("0x{}", &hash[hash.len()-40..])
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn sign_message(&self, data: &[u8]) -> Vec<u8> {
        use ed25519_dalek::SigningKey;
        let sk = SigningKey::from_bytes(&self.private_key.clone().try_into().unwrap());
        sk.sign(data).to_bytes().to_vec()
    }
}
