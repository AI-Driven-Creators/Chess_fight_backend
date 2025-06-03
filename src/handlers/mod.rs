use crate::types::response::{WsRequest, WsResponse};
use async_trait::async_trait;


// // 同步
// pub trait SyncMessageHandler: Send + Sync {
//     fn handle(&self, message: &WsRequest) -> WsResponse;
//     fn can_handle(&self, action: &str) -> bool;
// }

// 非同步
#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle(&self, val: &WsRequest) -> WsResponse;
    fn can_handle(&self, action: &str) -> bool;
}

pub mod echo;
pub mod ping;
pub mod unknown;
pub mod buy_xp;
pub mod shop;
pub mod create_game;
pub mod game_state_message_handler;


pub use echo::EchoHandler;
pub use ping::PingHandler;
pub use unknown::UnknownHandler;
pub use buy_xp::BuyXPHandler;
pub use shop::ShopHandler;
pub use create_game::CreateGameHandler;
pub use game_state_message_handler::GameStateMessageHandler;