use std::{
    net::{TcpListener, TcpStream},
    thread::spawn,
    sync::Arc,
};
use log::*;
use tungstenite::{accept, Error, Message, Result, handshake::HandshakeError, handshake::HandshakeRole};

mod handlers;
use handlers::{EchoHandler, PingHandler};

mod router;
use router::Router;

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

fn handle_client(stream: TcpStream) -> Result<()> {
    let mut router = Router::new();
    
    // 註冊處理器
    router.add_handler(Arc::new(EchoHandler));
    router.add_handler(Arc::new(PingHandler));

    let mut socket = accept(stream).map_err(must_not_block)?;
    info!("Client connected");

    loop {
        match socket.read()? {
            Message::Text(text) => {
                let response = match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(val) => {
                        match val.get("action").and_then(|a| a.as_str()) {
                            Some(action) => router.handle(action, &val),
                            None => serde_json::json!({ "status": "error", "reason": "missing action" }),
                        }
                    }
                    Err(_) => serde_json::json!({ "status": "error", "reason": "invalid json" }),
                };

                let response_text = response.to_string();
                socket.send(Message::Text(response_text))?;
            }
            Message::Binary(_) => {
                socket.send(Message::Text(
                    r#"{"status":"error","reason":"binary not supported"}"#.into(),
                ))?;
            }
            Message::Close(_) => {
                info!("Connection closed by client");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn main() {
    env_logger::init();
    let server = TcpListener::bind("127.0.0.1:9002").unwrap();
    info!("WebSocket server running on ws://127.0.0.1:9002");

    for stream in server.incoming() {
        spawn(move || match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream) {
                    match err {
                        Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                        e => error!("WebSocket error: {}", e),
                    }
                }
            }
            Err(e) => error!("TCP accept error: {}", e),
        });
    }
}
