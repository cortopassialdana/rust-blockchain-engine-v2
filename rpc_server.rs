use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RpcRequest {
    method: String,
    params: Vec<String>,
    id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RpcResponse {
    result: String,
    error: Option<String>,
    id: u64,
}

pub struct RpcServer;

impl RpcServer {
    pub fn new() -> Self {
        RpcServer
    }

    pub fn handle_request(&self, req: RpcRequest) -> RpcResponse {
        let result = match req.method.as_str() {
            "block_height" => "100".to_string(),
            "peer_count" => "8".to_string(),
            _ => "unknown method".to_string(),
        };
        RpcResponse {
            result,
            error: None,
            id: req.id,
        }
    }

    pub fn start(&self) {
        println!("RPC server started on 0.0.0.0:8545");
    }
}
