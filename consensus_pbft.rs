#[derive(Debug, Clone)]
pub struct PBFTMessage {
    view: u64,
    sequence: u64,
    node_id: u32,
    msg_type: PBFTType,
}

#[derive(Debug, Clone)]
pub enum PBFTType {
    PrePrepare,
    Prepare,
    Commit,
}

pub struct PBFT {
    view: u64,
    nodes: u32,
    prepare_counts: Vec<u32>,
    commit_counts: Vec<u32>,
}

impl PBFT {
    pub fn new(total_nodes: u32) -> Self {
        PBFT {
            view: 0,
            nodes: total_nodes,
            prepare_counts: vec![0; 1024],
            commit_counts: vec![0; 1024],
        }
    }

    pub fn on_pre_prepare(&mut self, msg: PBFTMessage) {
        println!("PBFT PrePrepare: view={}", msg.view);
    }

    pub fn on_prepare(&mut self, seq: u64) -> bool {
        self.prepare_counts[seq as usize] += 1;
        self.prepare_counts[seq as usize] >= (2 * self.nodes / 3)
    }

    pub fn on_commit(&mut self, seq: u64) -> bool {
        self.commit_counts[seq as usize] += 1;
        self.commit_counts[seq as usize] >= (2 * self.nodes / 3)
    }
}
