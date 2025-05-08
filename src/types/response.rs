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

    pub fn invalid_json() -> Self {
        Self::error("invalid json".to_string())
    }

    pub fn internal_server_error() -> Self {
        Self::error("internal server error".to_string())
    }

    pub fn missing_action() -> Self {
        Self::error("missing action".to_string())
    }

    pub fn unknown_action(action: &str) -> Self {
        Self::error(format!("unknown action: {}", action))
    }

    pub fn binary_not_supported() -> Self {
        Self::error("binary not supported".to_string())
    }
}
