use std::net::TcpStream;
use tungstenite::{accept, Error, Message, Result, handshake::HandshakeError, handshake::HandshakeRole};
use log::*;
use crate::router::Router;

pub fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

pub fn handle_client(stream: TcpStream, router: Router) -> Result<()> {
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