use super::transaction_model::Transaction;
use std::collections::VecDeque;

pub struct Mempool {
    queue: VecDeque<Transaction>,
    max_size: usize,
}

impl Mempool {
    pub fn new(max_size: usize) -> Self {
        Mempool {
            queue: VecDeque::new(),
            max_size,
        }
    }

    pub fn add_tx(&mut self, tx: Transaction) -> bool {
        if self.queue.len() >= self.max_size {
            return false;
        }
        self.queue.push_back(tx);
        true
    }

    pub fn remove_tx(&mut self, tx_id: &str) {
        self.queue.retain(|t| t.tx_id != tx_id);
    }

    pub fn get_batch(&mut self, count: usize) -> Vec<Transaction> {
        let mut res = Vec::new();
        for _ in 0..count {
            if let Some(tx) = self.queue.pop_front() {
                res.push(tx);
            } else {
                break;
            }
        }
        res
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }
}
