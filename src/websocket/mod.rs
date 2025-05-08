use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Message, Result},
};
use futures_util::{SinkExt, StreamExt};
use log::*;
use crate::router::Router;
use crate::types::response::{WsRequest, WsResponse};

pub async fn handle_client(stream: TcpStream, router: Router) -> Result<()> {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Client connected: {}", addr);

    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake occurred");
    info!("WebSocket connection established: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg? {
            Message::Text(text) => {
                let response = match serde_json::from_str::<WsRequest>(&text) {
                    Ok(request) => router.handle(&request.action, &request),
                    Err(_) => WsResponse::invalid_json(),
                };

                match serde_json::to_string(&response) {
                    Ok(response_text) => {
                        write.send(Message::Text(response_text)).await?;
                    }
                    Err(e) => {
                        error!("Failed to serialize response: {}", e);
                        let error_response = WsResponse::internal_server_error();
                        let error_text = serde_json::to_string(&error_response)
                            .unwrap_or_else(|_| serde_json::to_string(&WsResponse::internal_server_error())
                                .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string()));
                        write.send(Message::Text(error_text)).await?;
                    }
                }
            }
            Message::Binary(_) => {
                let response = WsResponse::binary_not_supported();
                match serde_json::to_string(&response) {
                    Ok(response_text) => {
                        write.send(Message::Text(response_text)).await?;
                    }
                    Err(e) => {
                        error!("Failed to serialize response: {}", e);
                        let error_response = WsResponse::internal_server_error();
                        let error_text = serde_json::to_string(&error_response)
                            .unwrap_or_else(|_| serde_json::to_string(&WsResponse::internal_server_error())
                                .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string()));
                        write.send(Message::Text(error_text)).await?;
                    }
                }
            }
            Message::Close(_) => {
                info!("Connection closed by client: {}", addr);
                break;
            }
            _ => {}
        }
    }

    Ok(())
} 