use super::transaction_model::Transaction;
use super::digital_signature::SignatureManager;
use ed25519_dalek::VerifyingKey;

pub struct BatchVerify;

impl BatchVerify {
    pub fn verify_transactions(
        txs: &[Transaction],
        pub_keys: &[VerifyingKey]
    ) -> Vec<bool> {
        let mut results = Vec::new();
        for (tx, pk) in txs.iter().zip(pub_keys.iter()) {
            let data = Self::tx_to_data(tx);
            let ok = SignatureManager::verify_data(pk, &data, &tx.signature);
            results.push(ok);
        }
        results
    }

    fn tx_to_data(tx: &Transaction) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(tx.from.as_bytes());
        data.extend_from_slice(tx.to.as_bytes());
        data.extend_from_slice(&tx.amount.to_be_bytes());
        data
    }
}
