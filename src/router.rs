use std::sync::Arc;
use serde_json::Value;
use crate::handlers::MessageHandler;

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

    pub fn handle(&self, action: &str, val: &Value) -> Value {
        for handler in &self.handlers {
            if handler.can_handle(action) {
                return handler.handle(val);
            }
        }
        
        // 如果沒有找到處理器，返回錯誤
        serde_json::json!({
            "status": "error",
            "reason": format!("unknown action: {}", action)
        })
    }
}
