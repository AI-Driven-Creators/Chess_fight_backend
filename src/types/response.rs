use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WsRequest {
    pub action: String,
    pub data: Value,
}

#[derive(Debug, Serialize)]
pub struct WsResponse {
    pub status: String,
    pub data: Option<Value>,
    pub error: Option<String>,
}

impl WsResponse {
    pub fn ok(data: Option<Value>) -> Self {
        Self {
            status: "ok".to_string(),
            data,
            error: None,
        }
    }

    pub fn error(reason: String) -> Self {
        Self {
            status: "error".to_string(),
            data: None,
            error: Some(reason),
        }
    }
}
