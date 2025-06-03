// 後端純資料結構定義
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChessPiece {
    pub name: String,
    pub cost: u32,
    pub level: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerGameState {
    pub player_id: String,
    pub money: u32,
    pub experience: u32,
    pub bench: Vec<ChessPiece>,
}


// use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct GameState {
    pub round: u32,
    pub money: u32,
    pub playerId: String,
    pub board: Vec<UnitOnBoard>,
    pub bench: Vec<UnitOnBench>,
    pub shop: Vec<ShopUnit>,
    pub synergies: Vec<Synergy>,
    pub level: u32,
    pub xp: XpInfo,
}

#[derive(Debug, Serialize)]
pub struct UnitOnBoard {
    pub id: String,
    pub chess: String,
    pub level: u32,
    pub position: [u32; 2],
}

#[derive(Debug, Serialize)]
pub struct UnitOnBench {
    pub id: String,
    pub chess: String,
    pub level: u32,
}

#[derive(Debug, Serialize)]
pub struct ShopUnit {
    pub chess: String,
    pub level: u32,
}

#[derive(Debug, Serialize)]
pub struct Synergy {
    pub name: String,
    pub count: u32,
    pub bonusLevel: u32,
}

#[derive(Debug, Serialize)]
pub struct XpInfo {
    pub current: u32,
    pub required: u32,
}