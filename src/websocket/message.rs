use crate::router::Router;
use crate::types::response::{WsRequest, WsResponse};
use futures_util::SinkExt;
use log::*;
use tokio_tungstenite::tungstenite::{Error, Message, Result};

pub async fn handle_text_message(
    text: &str,
    router: &Router,
    write: &mut (impl SinkExt<Message, Error = Error> + Unpin),
) -> Result<()> {
    let response = match serde_json::from_str::<WsRequest>(text) {
        Ok(request) => router.handle(&request.action, &request),
        Err(_) => WsResponse::invalid_json(),
    };

    let response_text = serde_json::to_string(&response)
        .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string());
    write.send(Message::Text(response_text)).await?;
    Ok(())
}

pub async fn handle_binary_message(
    write: &mut (impl SinkExt<Message, Error = Error> + Unpin),
) -> Result<()> {
    let response = WsResponse::binary_not_supported();
    let response_text = serde_json::to_string(&response)
        .unwrap_or_else(|_| r#"{"status":"error","error":"internal server error"}"#.to_string());
    write.send(Message::Text(response_text)).await?;
    Ok(())
}

pub async fn send_timeout_message(
    write: &mut (impl SinkExt<Message, Error = Error> + Unpin),
) -> Result<()> {
    let timeout_msg = WsResponse::error("connection timeout".to_string());
    let timeout_text = serde_json::to_string(&timeout_msg)
        .unwrap_or_else(|_| r#"{"status":"error","error":"timeout"}"#.to_string());
    write.send(Message::Text(timeout_text)).await?;
    Ok(())
}
