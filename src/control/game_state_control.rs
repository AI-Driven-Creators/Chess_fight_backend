// Control 可調用data，並控制記憶體中的狀態

use crate::types::game_state::{PlayerGameState, ChessPiece};
use crate::data::game_data::{all_chess_pieces, initial_money, initial_experience};   // 所有棋子資料來源
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;
use rand::thread_rng;  // 提供亂數生成器
use serde_json::json;  // 用來建立 JSON 物件

pub struct GameStateControl {
    state: Mutex<HashMap<String, PlayerGameState>>,
}

impl GameStateControl {

    pub async fn handle(player_id: &str) -> serde_json::Value {

		let mut rng = thread_rng(); // 建立亂數產生器
		let chess_pool = all_chess_pieces(); // 取得所有棋子

		// 隨機 bench（備戰區）
		let bench: Vec<_> = chess_pool
		.choose_multiple(&mut rng, 1)  // 從 chess_pool 中隨機挑選 1 個棋子
		.enumerate()  // 為選到的棋子加上索引（從 0 開始）
		.map(|(i, cp)| {  // 將每個棋子轉為 JSON 格式的物件
			json!({
				"id": format!("u00{}", i + 2),
				"chess": cp.name,
				"level": 1
			})
		})
		.collect();  // 收集成一個 Vec<Value> 陣列

		// 隨機 shop（商店 5 個）
		let shop: Vec<_> = chess_pool
			.choose_multiple(&mut rng, 5)
			.map(|cp| {
				json!({
					"chess": cp.name,
					"level": 1
				})
			})
			.collect();

		// synergy（羈絆）
		// 定義一組假資料：玩家上了 3 個「Warrior」，觸發了 1 級加成
		let synergies = vec![
			json!({
				"name": "Warrior",
				"count": 3,
				"bonusLevel": 1
			})
		];

		// 組合整個遊戲狀態的 JSON 資料
		json!({
			"round": 1,
			"money": initial_money(),
			"board": "待更新",
			"bench": bench,
			"shop": shop,
			"synergies": synergies,
			"level": 0,
			"xp": {
				"current": initial_experience(),
				"required": 6}
		})
    }
}
