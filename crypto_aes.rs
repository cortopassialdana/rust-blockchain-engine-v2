use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::Rng;

pub struct AesEncryptor;

impl AesEncryptor {
    pub fn generate_key() -> [u8;32] {
        let mut key = [0u8;32];
        rand::thread_rng().fill(&mut key);
        key
    }

    pub fn encrypt(key: &[u8;32], data: &[u8]) -> Vec<u8> {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng());
        let encrypted = cipher.encrypt(&nonce, data).unwrap();
        let mut result = Vec::new();
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&encrypted);
        result
    }

    pub fn decrypt(key: &[u8;32], data: &[u8]) -> Option<Vec<u8>> {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        cipher.decrypt(nonce, ciphertext).ok()
    }
}
