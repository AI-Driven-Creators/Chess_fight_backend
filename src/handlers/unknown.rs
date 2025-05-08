use crate::types::response::{WsRequest, WsResponse};
use super::MessageHandler;

pub struct UnknownHandler;

impl MessageHandler for UnknownHandler {
    fn handle(&self, val: &WsRequest) -> WsResponse {
        WsResponse::error(format!("unknown action: {}", val.action))
    }

    fn can_handle(&self, _action: &str) -> bool {
        true // 作為默認處理器
    }
} 