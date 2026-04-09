use std::fs::write;

pub struct PemWriter;

impl PemWriter {
    pub fn save_secret_key(path: &str, key: &[u8]) -> std::io::Result<()> {
        let pem = format!(
            "-----BEGIN ED25519 PRIVATE KEY-----\n{}\n-----END ED25519 PRIVATE KEY-----",
            base64::encode(key)
        );
        write(path, pem)
    }

    pub fn save_public_key(path: &str, key: &[u8]) -> std::io::Result<()> {
        let pem = format!(
            "-----BEGIN ED25519 PUBLIC KEY-----\n{}\n-----END ED25519 PUBLIC KEY-----",
            base64::encode(key)
        );
        write(path, pem)
    }
}
