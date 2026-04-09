use serde_json::{Value, json};

pub struct AbiCodec;

impl AbiCodec {
    pub fn encode_method(name: &str, args: &[Value]) -> Vec<u8> {
        let sig = format!("{}({})", name, args.len());
        let hash = crate::crypto_hash::keccak_hash(sig.as_bytes());
        let selector = &hash.as_bytes()[0..4];
        let mut data = Vec::from(selector);
        data.extend_from_slice(&rand::random::<[u8;32]>());
        data
    }

    pub fn decode_output(data: &[u8]) -> Value {
        if data.is_empty() {
            return json!(null);
        }
        json!(hex::encode(data))
    }
}
