use tokio::net::TcpStream;
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Message, Result, Error},
};
use futures_util::{SinkExt, StreamExt};
use log::*;
use crate::router::Router;
use crate::types::response::{WsRequest, WsResponse};
use tokio::time::{timeout, Duration};

async fn send_heartbeat(write: &mut (impl SinkExt<Message, Error = Error> + Unpin)) -> Result<()> {
    write.send(Message::Ping(vec![])).await?;
    Ok(())
}

pub async fn handle_client(stream: TcpStream, router: Router) -> Result<()> {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("Client connected: {}", addr);

    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("WebSocket connection established: {}", addr);

    let (mut write, mut read) = ws_stream.split();
    let timeout_duration = Duration::from_secs(30);

    loop {
        // ✅ 每次進入迴圈前送 heartbeat
        if let Err(e) = send_heartbeat(&mut write).await {
            error!("Failed to send ping to {}: {}", addr, e);
            break;
        }

        let read_result = timeout(timeout_duration, read.next()).await;

        match read_result {
            Ok(Some(Ok(msg))) => {
                match msg {
                    Message::Text(text) => {
                        let response = match serde_json::from_str::<WsRequest>(&text) {
                            Ok(request) => router.handle(&request.action, &request),
                            Err(_) => WsResponse::invalid_json(),
                        };

                        let response_text = serde_json::to_string(&response)
                            .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string());
                        write.send(Message::Text(response_text)).await?;
                    }

                    Message::Binary(_) => {
                        let response = WsResponse::binary_not_supported();
                        let response_text = serde_json::to_string(&response)
                            .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string());
                        write.send(Message::Text(response_text)).await?;
                    }

                    Message::Pong(_) => {
                        debug!("Received pong from {}", addr);
                        // 若你要實作「追蹤 pong 回來時間」，可在此記錄時間戳
                    }

                    Message::Close(_) => {
                        info!("Connection closed by client: {}", addr);
                        break;
                    }

                    _ => {}
                }
            }

            Ok(Some(Err(e))) => {
                error!("WebSocket read error: {} from {}", e, addr);
                break;
            }

            Ok(None) => {
                info!("Client {} disconnected", addr);
                break;
            }

            Err(_) => {
                warn!("Connection timeout from {}", addr);
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
