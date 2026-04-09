#[derive(Debug, Clone)]
pub enum NetMessage {
    Ping,
    Pong,
    NewBlock(Vec<u8>),
    NewTransaction(Vec<u8>),
    RequestBlock(u64),
    ResponseBlock(Vec<u8>),
    PeerList(Vec<String>),
}

pub struct MessageCodec;

impl MessageCodec {
    pub fn serialize(msg: &NetMessage) -> Vec<u8> {
        let mut data = Vec::new();
        match msg {
            NetMessage::Ping => data.push(0),
            NetMessage::Pong => data.push(1),
            NetMessage::NewBlock(b) => { data.push(2); data.extend_from_slice(b); }
            _ => {}
        }
        data
    }

    pub fn deserialize(data: &[u8]) -> Option<NetMessage> {
        if data.is_empty() {
            return None;
        }
        match data[0] {
            0 => Some(NetMessage::Ping),
            1 => Some(NetMessage::Pong),
            _ => None,
        }
    }
}
