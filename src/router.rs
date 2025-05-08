use std::sync::Arc;
use crate::handlers::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};

pub struct Router {
    handlers: Vec<Arc<dyn MessageHandler>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Arc<dyn MessageHandler>) {
        self.handlers.push(handler);
    }

    pub fn handle(&self, action: &str, request: &WsRequest) -> WsResponse {
        for handler in &self.handlers {
            if handler.can_handle(action) {
                return handler.handle(request);
            }
        }
        
        // 如果沒有找到處理器，返回錯誤
        WsResponse::error(format!("unknown action: {}", action))
    }
}
