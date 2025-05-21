use crate::types::response::{WsRequest, WsResponse};

pub trait MessageHandler: Send + Sync {
    fn handle(&self, val: &WsRequest) -> WsResponse;
    fn can_handle(&self, action: &str) -> bool;
}

pub mod echo;
pub mod ping;
pub mod unknown;

pub use echo::EchoHandler;
pub use ping::PingHandler;
pub use unknown::UnknownHandler;
