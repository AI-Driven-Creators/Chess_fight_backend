use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use serde_json::json;

pub struct ShopHandler;

impl MessageHandler for ShopHandler {
    fn handle(&self, val: &WsRequest) -> WsResponse {
        let response_payload = json!({
            "playerId": "p1",
            "success": true,
            "shop": [
                { "chess": "Mage", "level": 1 },
                { "chess": "Tank", "level": 1 },
                { "chess": "Knight", "level": 2 },
                { "chess": "Priest", "level": 1 },
                { "chess": "Hunter", "level": 1 }
            ],
            "money": 8
        });

        WsResponse {
            type_: "RefreshShopResult".to_string(),
            payload: Some(response_payload),
        }
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "RefreshShop"
    }
}
