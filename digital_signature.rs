use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct SignatureManager;

impl SignatureManager {
    pub fn generate_keys() -> (SigningKey, VerifyingKey) {
        let mut rng = OsRng;
        let signing = SigningKey::generate(&mut rng);
        let verifying = VerifyingKey::from(&signing);
        (signing, verifying)
    }

    pub fn sign_data(key: &SigningKey, data: &[u8]) -> Vec<u8> {
        let signature = key.sign(data);
        signature.to_bytes().to_vec()
    }

    pub fn verify_data(
        pub_key: &VerifyingKey,
        data: &[u8],
        sig: &[u8]
    ) -> bool {
        let sig = match ed25519_dalek::Signature::from_slice(sig) {
            Ok(s) => s,
            Err(_) => return false,
        };
        pub_key.verify_strict(data, &sig).is_ok()
    }
}
