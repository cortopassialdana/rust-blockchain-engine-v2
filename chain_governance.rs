use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Proposal {
    id: String,
    title: String,
    votes_for: u64,
    votes_against: u64,
    active: bool,
}

pub struct ChainGovernance {
    proposals: HashMap<String, Proposal>,
    voters: HashMap<String, u64>,
}

impl ChainGovernance {
    pub fn new() -> Self {
        ChainGovernance {
            proposals: HashMap::new(),
            voters: HashMap::new(),
        }
    }

    pub fn create_proposal(&mut self, id: String, title: String) {
        let p = Proposal {
            id: id.clone(),
            title,
            votes_for: 0,
            votes_against: 0,
            active: true,
        };
        self.proposals.insert(id, p);
    }

    pub fn vote(&mut self, pid: &str, addr: &str, approve: bool, power: u64) {
        if let Some(p) = self.proposals.get_mut(pid) {
            if approve {
                p.votes_for += power;
            } else {
                p.votes_against += power;
            }
        }
    }
}
