use std::sync::Arc;
use tokio::net::TcpListener;
use log::*;
use tokio_tungstenite::tungstenite::Error;

mod websocket;
mod handlers;
mod router;
mod types;

use websocket::handle_client;
use handlers::{EchoHandler, PingHandler, UnknownHandler};
use router::Router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(addr).await?;
    info!("WebSocket server running on ws://{}", addr);

    let mut router = Router::new();
    
    // 註冊處理器
    router.add_handler(Arc::new(EchoHandler));
    router.add_handler(Arc::new(PingHandler));
    router.add_handler(Arc::new(UnknownHandler));

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from: {}", addr);
        
        let router = router.clone();
        tokio::spawn(async move {
            if let Err(err) = handle_client(stream, router).await {
                match err {
                    Error::ConnectionClosed | 
                    Error::Protocol(_) | 
                    Error::Utf8 => (),
                    e => error!("WebSocket error: {}", e),
                }
            }
        });
    }

    Ok(())
}
