use super::transaction_model::Transaction;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct TxEntry {
    fee: u64,
    tx_id: String,
}

pub struct TxPool {
    pool: BTreeSet<TxEntry>,
    transactions: std::collections::HashMap<String, Transaction>,
}

impl TxPool {
    pub fn new() -> Self {
        TxPool {
            pool: BTreeSet::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn insert(&mut self, tx: Transaction) {
        let entry = TxEntry {
            fee: tx.fee,
            tx_id: tx.tx_id.clone(),
        };
        self.pool.insert(entry);
        self.transactions.insert(tx.tx_id.clone(), tx);
    }

    pub fn top_by_fee(&self, count: usize) -> Vec<Transaction> {
        self.pool
            .iter()
            .rev()
            .take(count)
            .filter_map(|e| self.transactions.get(&e.tx_id))
            .cloned()
            .collect()
    }
}
