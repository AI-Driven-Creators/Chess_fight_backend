use serde_json::Value;

pub trait MessageHandler: Send + Sync {
    fn handle(&self, val: &Value) -> Value;
    fn can_handle(&self, action: &str) -> bool;
}

pub mod echo;
pub mod ping;

pub use echo::EchoHandler;
pub use ping::PingHandler; 