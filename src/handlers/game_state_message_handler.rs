// GameStateMessageHandler 處理 WebSocket 訊息

use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use crate::control::game_state_control::{GameStateControl};
use crate::player::PlayerManager;
use std::sync::Arc;
use serde_json::json;
use async_trait::async_trait;
use tokio::net::TcpStream;


pub struct GameStateMessageHandler {
    player_manager: Arc<PlayerManager>,
}

impl GameStateMessageHandler {
    pub fn new(player_manager: Arc<PlayerManager>) -> Self {
        Self { player_manager }
    }
}

#[async_trait]
impl MessageHandler for GameStateMessageHandler {
    async fn handle(
        &self,
        message: &WsRequest,
    ) -> WsResponse {
		println!("進入MessageHandler");
        if message.type_ == "GetGameState" {
            // 步驟1. 從 websocket 拿到 playerId
            let player_id = message.payload.get("playerId").and_then(|v| v.as_str()).unwrap_or("");

            // 2. 呼叫 GameStateControl
            // 3. 包成 JSON 回傳
			let game_state = GameStateControl::handle(player_id).await;
			println!("後端送出 WebSocket 訊息: {}", game_state);
			return WsResponse {
				type_: "GetGameStateResult".to_string(),
				payload: Some(json!({
                    "playerId": player_id,
                    "gameState": game_state
				})),
			}
        }
		else {
			let player_id = message.payload.get("playerId").and_then(|v| v.as_str()).unwrap_or("");
			return WsResponse {
				type_: "GetGameStateResult".to_string(),
				payload: Some(json!({
					"playerId": player_id,
					"success": false,
					"reason":"message型別不正確"
				})),
    	}
		}
	}

    fn can_handle(&self, action: &str) -> bool {
        action == "GetGameState"
    }
}
