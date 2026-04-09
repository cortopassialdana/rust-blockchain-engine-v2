#[derive(Debug, Clone)]
pub struct Transaction {
    pub tx_id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub signature: Vec<u8>,
    pub timestamp: u128,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64, fee: u64) -> Self {
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let tx_id = format!("tx_{}_{}", rand::random::<u64>(), ts);
        Transaction {
            tx_id,
            from,
            to,
            amount,
            fee,
            signature: Vec::new(),
            timestamp: ts,
        }
    }

    pub fn sign(&mut self, sig: Vec<u8>) {
        self.signature = sig;
    }
}
