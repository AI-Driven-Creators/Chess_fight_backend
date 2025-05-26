use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};

pub struct EchoHandler;

impl MessageHandler for EchoHandler {
    fn handle(&self, val: &WsRequest) -> WsResponse {
        WsResponse::ok(Some(val.payload.clone()))
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "echo"
    }
}
