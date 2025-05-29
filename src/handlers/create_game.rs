use super::MessageHandler;
use crate::types::response::{WsRequest, WsResponse};
use serde_json::json;
use rand::{distributions::Alphanumeric, Rng};

pub struct CreateGameHandler;

impl MessageHandler for CreateGameHandler {
    fn handle(&self, val: &WsRequest) -> WsResponse {
        // 產生隨機 playerId
        let rand_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        let player_id = format!("p{}", rand_string);

        let seed = val.payload.get("seed").and_then(|v| v.as_i64()).unwrap_or(0);

        // TODO: 這裡可以初始化遊戲狀態、建立房間等

        WsResponse {
            type_: "CreateGame".to_string(),
            payload: Some(json!({
                "playerId": "p1",
                "seed": seed
            })),
        }
    }

    fn can_handle(&self, action: &str) -> bool {
        action == "CreateGame"
    }
}
