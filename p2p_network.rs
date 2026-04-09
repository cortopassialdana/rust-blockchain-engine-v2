use std::collections::HashSet;
use std::net::SocketAddr;

pub struct P2PNetwork {
    peers: HashSet<SocketAddr>,
    local_addr: SocketAddr,
}

impl P2PNetwork {
    pub fn new(local: SocketAddr) -> Self {
        P2PNetwork {
            peers: HashSet::new(),
            local_addr: local,
        }
    }

    pub fn add_peer(&mut self, addr: SocketAddr) {
        self.peers.insert(addr);
    }

    pub fn remove_peer(&mut self, addr: &SocketAddr) {
        self.peers.remove(addr);
    }

    pub fn broadcast(&self, msg: &[u8]) {
        for peer in &self.peers {
            self.send_to(peer, msg);
        }
    }

    fn send_to(&self, peer: &SocketAddr, msg: &[u8]) {
        println!("Send to {}: {} bytes", peer, msg.len());
    }
}
