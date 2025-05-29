use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use crate::player::PlayerManager;
use serde_json::json;
use std::sync::Arc;

pub struct BuyXPHandler {
    player_manager: Arc<PlayerManager>,
}

impl BuyXPHandler {
    pub fn new(player_manager: Arc<PlayerManager>) -> Self {
        Self { player_manager }
    }
}

impl MessageHandler for BuyXPHandler {
    fn handle(&self, val: &WsRequest) -> WsResponse {
        // 从请求中获取 playerId
        let player_id = match val.payload.get("playerId") {
            Some(id) => match id.as_str() {
                Some(id_str) => id_str,
                None => return WsResponse::error("invalid playerId format".to_string()),
            },
            None => return WsResponse::error("missing playerId".to_string()),
        };

        // 尝试购买经验值
        match self.player_manager.buy_xp(player_id) {
            Ok(player) => {
                WsResponse {
                    type_: "BuyXPResult".to_string(),
                    payload: Some(json!({
                        "playerId": player.id,
                        "success": true,
                        "money": player.money,
                        "xp": {
                            "current": player.xp.current,
                            "required": player.xp.required
                        }
                    })),
                }
            }
            Err(reason) => {
                WsResponse {
                    type_: "BuyXPResult".to_string(),
                    payload: Some(json!({
                        "playerId": player_id,
                        "success": false,
                        "reason": reason
                    })),
                }
            }
        }
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "BuyXP"
    }
} 