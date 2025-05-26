use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct WsRequest {
    #[serde(rename = "type")]
    pub type_: String,
    pub payload: Value,
}

#[derive(Debug, Serialize)]
pub struct WsResponse {
    #[serde(rename = "type")]
    pub type_: String,
    pub payload: Option<Value>,
}

impl WsResponse {
    pub fn ok(data: Option<Value>) -> Self {
        Self {
            type_: "Success".to_string(),
            payload: data,
        }
    }

    pub fn error(reason: String) -> Self {
        Self {
            type_: "Error".to_string(),
            payload: Some(serde_json::json!({ "error": reason })),
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
