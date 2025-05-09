use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Message, Result},
};
use futures_util::{SinkExt, StreamExt};
use log::*;
use crate::router::Router;
use crate::types::response::{WsRequest, WsResponse};
use tokio::time::{timeout, Duration};

pub async fn handle_client(stream: TcpStream, router: Router) -> Result<()> {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Client connected: {}", addr);

    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake occurred");
    info!("WebSocket connection established: {}", addr);

    let (mut write, mut read) = ws_stream.split();
    let timeout_duration = Duration::from_secs(30);

    loop {
        let read_result = timeout(timeout_duration, read.next()).await;

        match read_result {
            Ok(Some(Ok(msg))) => {
                match msg {
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
                                    .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string());
                                write.send(Message::Text(error_text)).await?;
                            }
                        }
                    }
                    Message::Binary(_) => {
                        let response = WsResponse::binary_not_supported();
                        let response_text = serde_json::to_string(&response)
                            .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string());
                        write.send(Message::Text(response_text)).await?;
                    }
                    Message::Close(_) => {
                        info!("Connection closed by client: {}", addr);
                        break;
                    }
                    _ => {}
                }
            }

            Ok(Some(Err(e))) => {
                error!("WebSocket read error: {}", e);
                break;
            }

            Ok(None) => {
                // stream closed
                info!("WebSocket stream ended: {}", addr);
                break;
            }

            Err(_) => {
                // timeout 發生
                warn!("Connection timeout: {}", addr);
                let timeout_msg = WsResponse::error("connection timeout".to_string());
                let timeout_text = serde_json::to_string(&timeout_msg)
                    .unwrap_or_else(|_| r#"{"status":"error","error":"timeout"}"#.to_string());
                write.send(Message::Text(timeout_text)).await?;
                break;
            }
        }
    }

    Ok(())
}
