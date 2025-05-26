use crate::handlers::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use std::sync::Arc;

#[derive(Clone)]
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

        // 如果没有找到处理器，返回错误
        WsResponse::error(format!("unknown action: {}", action))
    }
}
