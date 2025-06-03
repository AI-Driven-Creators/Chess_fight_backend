// Control 可調用data，並控制記憶體中的狀態

use crate::types::game_state::{PlayerGameState, ChessPiece};
use crate::data::game_data::{all_chess_pieces, initial_money, initial_experience};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde_json::json;

pub struct GameStateControl {
    state: Mutex<HashMap<String, PlayerGameState>>,
}

impl GameStateControl {

    pub async fn handle(player_id: &str) -> serde_json::Value {
        // 模擬生成資料
        json!({
            "level": 3,
            "gold": 10,
            "units": ["劍士", "弓箭手"],
            "playerId": player_id
        })
    }

    pub fn new() -> Self {
        Self {
            state: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_or_create_player_state(&self, player_id: &str) -> PlayerGameState {
        let mut state = self.state.lock().unwrap();
        state.entry(player_id.to_string()).or_insert_with(|| {
            let mut rng = thread_rng();
            let bench = all_chess_pieces()
                .choose_multiple(&mut rng, 5)
                .cloned()
                .collect();

            PlayerGameState {
                player_id: player_id.to_string(),
                money: initial_money(),
                experience: initial_experience(),
                bench,
            }
        }).clone()
    }

    pub fn update_state(&self, player_state: PlayerGameState) {
        let mut state = self.state.lock().unwrap();
        state.insert(player_state.player_id.clone(), player_state);
    }
}

