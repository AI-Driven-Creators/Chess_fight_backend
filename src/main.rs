use std::{
    net::{TcpListener, TcpStream},
    thread::spawn,
};
use log::*;
use tungstenite::{Error, HandshakeError, Message, Result, accept, handshake::HandshakeRole};

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

fn handle_client(stream: TcpStream) -> Result<()> {
    let mut socket = accept(stream).map_err(must_not_block)?;
    info!("Client connected");

    loop {
        match socket.read()? {
            Message::Text(text) => {
                let response = match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(val) => {
                        match val.get("action").and_then(|a| a.as_str()) {
                            Some("echo") => {
                                let msg = val.get("data").unwrap_or(&serde_json::Value::Null);
                                serde_json::json!({ "status": "ok", "echo": msg }).to_string()
                            }
                            Some(other) => {
                                serde_json::json!({ "status": "error", "reason": format!("unknown action: {}", other) }).to_string()
                            }
                            None => serde_json::json!({ "status": "error", "reason": "missing action" }).to_string(),
                        }
                    }
                    Err(_) => serde_json::json!({ "status": "error", "reason": "invalid json" }).to_string(),
                };
                let response_text = response.into();
                socket.send(Message::Text(response_text))?;
            }
            Message::Binary(_) => {
                socket.send(Message::Text(
                    (r#"{"status":"error","reason":"binary not supported"}"#.to_string()).into(),
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

    for stream in server.incoming() {
        spawn(move || match stream {
            Ok(stream) => {
                if let Err(err) = handle_client(stream) {
                    match err {
                        Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
                        e => error!("test: {}", e),
                    }
                }
            }
            Err(e) => error!("Error accepting stream: {}", e),
        });
    }
}
