use std::{
    net::TcpListener,
    thread::spawn,
    sync::Arc,
};
use log::*;
use tungstenite::Error;

mod websocket;
mod handlers;
mod router;
mod types;

use websocket::handle_client;
use handlers::{EchoHandler, PingHandler, UnknownHandler};
use router::Router;

fn main() {
    env_logger::init();
    let server = TcpListener::bind("127.0.0.1:9002").unwrap();
    info!("WebSocket server running on ws://127.0.0.1:9002");

    for stream in server.incoming() {
        let mut router = Router::new();
        
        // 註冊處理器
        router.add_handler(Arc::new(EchoHandler));
        router.add_handler(Arc::new(PingHandler));
        router.add_handler(Arc::new(UnknownHandler));

        spawn(move || match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream, router) {
                    match err {
                        Error::ConnectionClosed | 
                        Error::Protocol(_) | 
                        Error::Utf8 => (),
                        e => error!("WebSocket error: {}", e),
                    }
                }
            }
            Err(e) => error!("TCP accept error: {}", e),
        });
    }
}
