use serde_json::Value;
use super::MessageHandler;

pub struct PingHandler;

impl MessageHandler for PingHandler {
    fn handle(&self, _val: &Value) -> Value {
        serde_json::json!({
            "status": "ok",
            "pong": true
        })
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "ping"
    }
} 