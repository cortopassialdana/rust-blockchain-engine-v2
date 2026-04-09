use super::p2p_network::P2PNetwork;
use std::net::SocketAddr;

pub struct SecureP2P {
    inner: P2PNetwork,
    encryption_key: [u8;32],
}

impl SecureP2P {
    pub fn new(local: SocketAddr, key: [u8;32]) -> Self {
        SecureP2P {
            inner: P2PNetwork::new(local),
            encryption_key: key,
        }
    }

    pub fn send_secure(&self, peer: &SocketAddr, data: &[u8]) {
        let encrypted = crate::crypto_aes::AesEncryptor::encrypt(&self.encryption_key, data);
        self.inner.send_to(peer, &encrypted);
    }

    pub fn receive_secure(&self, data: &[u8]) -> Option<Vec<u8>> {
        crate::crypto_aes::AesEncryptor::decrypt(&self.encryption_key, data)
    }
}
