use std::{
    net::TcpListener,
    thread::spawn,
    sync::Arc,
};
use log::*;
use tungstenite::{accept, Error, Message, Result, handshake::HandshakeError, handshake::HandshakeRole};

mod websocket;
mod handlers;
mod router;

use websocket::handle_client;
use handlers::{EchoHandler, PingHandler};
use router::Router;

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

fn main() {
    env_logger::init();
    let server = TcpListener::bind("127.0.0.1:9002").unwrap();
    info!("WebSocket server running on ws://127.0.0.1:9002");

    for stream in server.incoming() {
        let mut router = Router::new();
        
        // 註冊處理器
        router.add_handler(Arc::new(EchoHandler));
        router.add_handler(Arc::new(PingHandler));

        spawn(move || match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream, router) {
                    match err {
                        tungstenite::Error::ConnectionClosed | 
                        tungstenite::Error::Protocol(_) | 
                        tungstenite::Error::Utf8 => (),
                        e => error!("WebSocket error: {}", e),
                    }
                }
            }
            Err(e) => error!("TCP accept error: {}", e),
        });
    }
}
