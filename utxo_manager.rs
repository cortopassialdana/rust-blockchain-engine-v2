use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Utxo {
    pub tx_id: String,
    pub index: u32,
    pub owner: String,
    pub amount: u64,
}

pub struct UtxoManager {
    utxos: HashMap<String, Utxo>,
}

impl UtxoManager {
    pub fn new() -> Self {
        UtxoManager {
            utxos: HashMap::new(),
        }
    }

    pub fn add_utxo(&mut self, utxo: Utxo) {
        let key = format!("{}:{}", utxo.tx_id, utxo.index);
        self.utxos.insert(key, utxo);
    }

    pub fn remove_utxo(&mut self, tx_id: &str, index: u32) {
        let key = format!("{}:{}", tx_id, index);
        self.utxos.remove(&key);
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.utxos
            .values()
            .filter(|u| u.owner == address)
            .map(|u| u.amount)
            .sum()
    }
}
