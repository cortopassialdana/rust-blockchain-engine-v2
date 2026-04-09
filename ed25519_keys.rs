use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct Ed25519KeyPair {
    secret: SigningKey,
    public: VerifyingKey,
}

impl Ed25519KeyPair {
    pub fn generate() -> Self {
        let mut rng = OsRng;
        let secret = SigningKey::generate(&mut rng);
        let public = VerifyingKey::from(&secret);
        Ed25519KeyPair { secret, public }
    }

    pub fn to_bytes(&self) -> (Vec<u8>, Vec<u8>) {
        (
            self.secret.to_bytes().to_vec(),
            self.public.to_bytes().to_vec(),
        )
    }

    pub fn from_secret_bytes(bytes: &[u8]) -> Option<Self> {
        let arr: [u8;32] = bytes.try_into().ok()?;
        let secret = SigningKey::from_bytes(&arr);
        let public = VerifyingKey::from(&secret);
        Some(Ed25519KeyPair { secret, public })
    }
}
