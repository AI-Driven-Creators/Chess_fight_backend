use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use serde_json::json;

pub struct BuyXPHandler;

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

        // TODO: 这里应该添加实际的游戏逻辑，比如检查玩家金钱、更新经验值等
        // 这里只是示例响应
        let success = true; // 假设购买成功

        if success {
            WsResponse::ok(Some(json!({
                "playerId": player_id,
                "success": true,
                "money": 4,
                "xp": {
                    "current": 6,
                    "required": 8
                }
            })))
        } else {
            WsResponse::ok(Some(json!({
                "playerId": player_id,
                "success": false,
                "reason": "not enough money"
            })))
        }
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "BuyXP"
    }
} 