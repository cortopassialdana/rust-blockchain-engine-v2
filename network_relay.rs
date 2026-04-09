use super::p2p_network::P2PNetwork;
use std::net::SocketAddr;

pub struct NetworkRelay {
    network: P2PNetwork,
    relay_peers: Vec<SocketAddr>,
}

impl NetworkRelay {
    pub fn new(network: P2PNetwork) -> Self {
        NetworkRelay {
            network,
            relay_peers: Vec::new(),
        }
    }

    pub fn add_relay(&mut self, addr: SocketAddr) {
        self.relay_peers.push(addr);
    }

    pub fn relay_message(&self, from: &SocketAddr, msg: &[u8]) {
        for peer in &self.relay_peers {
            if peer != from {
                self.network.send_to(peer, msg);
            }
        }
    }
}
