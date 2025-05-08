use serde_json::Value;

pub trait MessageHandler: Send + Sync {
    fn handle(&self, val: &Value) -> Value;
    fn can_handle(&self, action: &str) -> bool;
}

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

pub struct UnknownHandler;

impl MessageHandler for UnknownHandler {
    fn handle(&self, _val: &Value) -> Value {
        serde_json::json!({
            "status": "error",
            "reason": "unknown action"
        })
    }

    fn can_handle(&self, _action: &str) -> bool {
        false
    }
}

pub fn get_handler(action: &str) -> Box<dyn MessageHandler> {
    if action == "echo" {
        Box::new(EchoHandler)
    } else if action == "ping" {
        Box::new(PingHandler)
    } else {
        Box::new(UnknownHandler)
    }
} 