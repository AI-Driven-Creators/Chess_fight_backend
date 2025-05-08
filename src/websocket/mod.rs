use std::net::TcpStream;
use tungstenite::{accept, Error, Message, Result, handshake::HandshakeError, handshake::HandshakeRole};
use log::*;
use crate::router::Router;
use crate::types::response::{WsRequest, WsResponse};

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
                let response = match serde_json::from_str::<WsRequest>(&text) {
                    Ok(request) => router.handle(&request.action, &request),
                    Err(_) => WsResponse::invalid_json(),
                };

                match serde_json::to_string(&response) {
                    Ok(response_text) => {
                        socket.send(Message::Text(response_text))?;
                    }
                    Err(e) => {
                        error!("Failed to serialize response: {}", e);
                        let error_response = WsResponse::internal_server_error();
                        let error_text = serde_json::to_string(&error_response)
                            .unwrap_or_else(|_| serde_json::to_string(&WsResponse::internal_server_error())
                                .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string()));
                        socket.send(Message::Text(error_text))?;
                    }
                }
            }
            Message::Binary(_) => {
                let response = WsResponse::binary_not_supported();
                match serde_json::to_string(&response) {
                    Ok(response_text) => {
                        socket.send(Message::Text(response_text))?;
                    }
                    Err(e) => {
                        error!("Failed to serialize response: {}", e);
                        let error_response = WsResponse::internal_server_error();
                        let error_text = serde_json::to_string(&error_response)
                            .unwrap_or_else(|_| serde_json::to_string(&WsResponse::internal_server_error())
                                .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string()));
                        socket.send(Message::Text(error_text))?;
                    }
                }
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