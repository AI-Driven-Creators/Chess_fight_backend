use serde_json::Value;
use super::MessageHandler;

pub struct EchoHandler;

impl MessageHandler for EchoHandler {
    fn handle(&self, val: &Value) -> Value {
        let msg = val.get("data").unwrap_or(&Value::Null);
        serde_json::json!({
            "status": "ok",
            "echo": msg
        })
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "echo"
    }
} 