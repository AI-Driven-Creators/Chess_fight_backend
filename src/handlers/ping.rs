use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};

pub struct PingHandler;

impl MessageHandler for PingHandler {
    fn handle(&self, _val: &WsRequest) -> WsResponse {
        WsResponse::ok(Some(serde_json::json!({ "pong": true })))
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "ping"
    }
}
