use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use async_trait::async_trait;

pub struct PingHandler;

#[async_trait]
impl MessageHandler for PingHandler {
    async fn handle(&self, _val: &WsRequest) -> WsResponse {
        WsResponse::ok(Some(serde_json::json!({ "pong": true })))
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "ping"
    }
}
