use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use async_trait::async_trait;

pub struct UnknownHandler;

#[async_trait]
impl MessageHandler for UnknownHandler {
    async fn handle(&self, val: &WsRequest) -> WsResponse {
        WsResponse::error(format!("unknown action: {}", val.type_))
    }

    fn can_handle(&self, _action: &str) -> bool {
        true // 作为默认处理器
    }
}
