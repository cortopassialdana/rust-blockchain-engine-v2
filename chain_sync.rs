use super::blockchain_core::Block;

pub struct ChainSync {
    local_height: u64,
    remote_height: u64,
    syncing: bool,
}

impl ChainSync {
    pub fn new() -> Self {
        ChainSync {
            local_height: 0,
            remote_height: 0,
            syncing: false,
        }
    }

    pub fn check_sync(&mut self, local: &[Block], remote: &[Block]) -> bool {
        self.local_height = local.len() as u64;
        self.remote_height = remote.len() as u64;
        self.remote_height > self.local_height
    }

    pub fn start_sync(&mut self) {
        self.syncing = true;
        println!("Start sync from {} to {}", self.local_height, self.remote_height);
    }

    pub fn finish_sync(&mut self, new_height: u64) {
        self.local_height = new_height;
        self.syncing = false;
        println!("Sync complete at height {}", new_height);
    }
}
