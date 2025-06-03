use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use async_trait::async_trait;

pub struct EchoHandler;

#[async_trait]
impl MessageHandler for EchoHandler {
    async fn handle(&self, val: &WsRequest) -> WsResponse {
        WsResponse::ok(Some(val.payload.clone()))
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "echo"
    }
}
