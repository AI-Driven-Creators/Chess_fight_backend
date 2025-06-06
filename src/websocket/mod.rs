use crate::router::Router;
use futures_util::{SinkExt, StreamExt};
use log::*;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};

mod heartbeat;
mod message;

use heartbeat::send_heartbeat;
use message::{handle_binary_message, handle_text_message, send_timeout_message};

pub async fn handle_client(stream: TcpStream, router: Router) -> Result<()> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Client connected: {}", addr);

    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("WebSocket connection established: {}", addr);

    let (mut write, mut read) = ws_stream.split();
    let timeout_duration = Duration::from_secs(30);

    loop {
        // 發送心跳
        if let Err(e) = send_heartbeat(&mut write).await {
            error!("Failed to send ping to {}: {}", addr, e);
            break;
        }

        // 等待消息
        let read_result = timeout(timeout_duration, read.next()).await;

        match read_result {
            Ok(Some(Ok(msg))) => match msg {
                Message::Text(text) => {
                    println!("收到前端文字訊息: {}", text);

                    if let Err(e) = handle_text_message(&text, &router, &mut write).await {
                        error!("Failed to handle text message from {}: {}", addr, e);
                        break;
                    }
                }
                Message::Binary(_) => {
                    println!("收到前端二進位訊息");

                    if let Err(e) = handle_binary_message(&mut write).await {
                        error!("Failed to handle binary message from {}: {}", addr, e);
                        break;
                    }
                }
                Message::Close(_) => {
                    info!("Client {} disconnected", addr);
                    break;
                }
                Message::Pong(_) => {
                    debug!("Received pong from {}", addr);
                }
                _ => {}
            },
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
                if let Err(e) = send_timeout_message(&mut write).await {
                    error!("Failed to send timeout message to {}: {}", addr, e);
                }
                break;
            }
        }
    }

    Ok(())
}
