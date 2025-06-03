use log::*;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Error;

mod handlers;
mod router;
mod types;
mod websocket;
mod player;
mod control;
mod data;

use handlers::{EchoHandler, PingHandler, UnknownHandler, BuyXPHandler, ShopHandler, CreateGameHandler, GameStateMessageHandler};
use router::Router;
use websocket::handle_client;
use player::PlayerManager;
use crate::control::game_state_control::GameStateControl;
use data::{all_chess_pieces,initial_money,initial_experience};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server running on ws://{}", addr);

    let mut router = Router::new();
    let player_manager = Arc::new(PlayerManager::new());

    // 註冊處理器
    router.add_handler(Arc::new(EchoHandler));
    router.add_handler(Arc::new(PingHandler));
    router.add_handler(Arc::new(BuyXPHandler::new(player_manager.clone())));
    router.add_handler(Arc::new(ShopHandler::new(player_manager.clone())));
    router.add_handler(Arc::new(CreateGameHandler));
    router.add_handler(Arc::new(GameStateMessageHandler::new(player_manager.clone())));
    router.add_handler(Arc::new(UnknownHandler));
    
    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from: {}", addr);

        let router = router.clone();
        tokio::spawn(async move {
            if let Err(err) = handle_client(stream, router).await {
                match err {
                    Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                    e => error!("WebSocket error: {}", e),
                }
            }
        });
    }

    Ok(())
}
