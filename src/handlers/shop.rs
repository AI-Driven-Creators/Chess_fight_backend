use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use crate::player::PlayerManager;
use serde_json::json;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::Arc;
use async_trait::async_trait;

pub struct ShopHandler {
    player_manager: Arc<PlayerManager>,
}

impl ShopHandler {
    pub fn new(player_manager: Arc<PlayerManager>) -> Self {
        Self { player_manager }
    }
}

const ALL_CHESS: &[&str] = &[
    "Mage", "Tank", "Knight", "Priest", "Hunter", "Assassin", "Archer", "Berserker",
    "Paladin", "Warlock", "Necromancer", "Druid", "Shaman", "Blademaster", "Sniper",
    "Engineer", "Beastmaster", "Phantom", "Guardian", "Elemental"
];

#[async_trait]
impl MessageHandler for ShopHandler {
    async fn handle(&self, val: &WsRequest) -> WsResponse {
        // 解析 playerId
        let player_id = match val.payload.get("playerId") {
            Some(id) => match id.as_str() {
                Some(id_str) => id_str,
                None => return WsResponse::error("invalid playerId format".to_string()),
            },
            None => return WsResponse::error("missing playerId".to_string()),
        };

        // 嘗試扣除金錢
        match self.player_manager.refresh_shop(player_id) {
            Ok(new_money) => {
                let mut rng = thread_rng();
                let selected_chess: Vec<_> = ALL_CHESS
                    .choose_multiple(&mut rng, 5)
                    .map(|&chess| json!({ "chess": chess, "level": 1 }))
                    .collect();

                WsResponse {
                    type_: "RefreshShopResult".to_string(),
                    payload: Some(json!({
                        "playerId": player_id,
                        "success": true,
                        "shop": selected_chess,
                        "money": new_money
                    })),
                }
            }
            Err(reason) => {
                WsResponse {
                    type_: "RefreshShopResult".to_string(),
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
        action == "RefreshShop"
    }
}
