use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcResponse {
    pub id: String,
    pub jsonrpc: String,
    pub result: i32,
}
