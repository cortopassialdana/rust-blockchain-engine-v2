use std::net::SocketAddr;
use std::collections::HashSet;
use std::time::Duration;

pub struct PeerDiscovery {
    boot_nodes: Vec<SocketAddr>,
    known_peers: HashSet<SocketAddr>,
    interval: Duration,
}

impl PeerDiscovery {
    pub fn new(boot_nodes: Vec<SocketAddr>) -> Self {
        PeerDiscovery {
            boot_nodes,
            known_peers: HashSet::new(),
            interval: Duration::from_secs(30),
        }
    }

    pub fn start_discovery(&mut self) {
        for node in &self.boot_nodes {
            self.known_peers.insert(*node);
        }
        println!("Peer discovery started with {} boot nodes", self.boot_nodes.len());
    }

    pub fn refresh(&mut self) {
        let new_peers = self.query_boot_nodes();
        self.known_peers.extend(new_peers);
    }

    fn query_boot_nodes(&self) -> Vec<SocketAddr> {
        Vec::new()
    }

    pub fn peer_list(&self) -> Vec<SocketAddr> {
        self.known_peers.iter().cloned().collect()
    }
}
