use std::collections::HashMap;

pub struct StakeConsensus {
    stakes: HashMap<String, u64>,
    total_stake: u64,
}

impl StakeConsensus {
    pub fn new() -> Self {
        StakeConsensus {
            stakes: HashMap::new(),
            total_stake: 0,
        }
    }

    pub fn delegate(&mut self, address: String, amount: u64) {
        *self.stakes.entry(address).or_insert(0) += amount;
        self.total_stake += amount;
    }

    pub fn undelegate(&mut self, address: &str, amount: u64) {
        if let Some(s) = self.stakes.get_mut(address) {
            *s = s.saturating_sub(amount);
            self.total_stake = self.total_stake.saturating_sub(amount);
        }
    }

    pub fn select_validator(&self) -> Option<String> {
        if self.total_stake == 0 {
            return None;
        }
        self.stakes.keys().next().cloned()
    }
}
