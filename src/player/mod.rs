use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub id: String,
    pub money: i32,
    pub xp: XPData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XPData {
    pub current: i32,
    pub required: i32,
}

pub struct PlayerManager {
    players: Arc<Mutex<HashMap<String, PlayerData>>>,
}

impl PlayerManager {
    pub fn new() -> Self {
        let mut map = HashMap::new();
    
        // 插入預設玩家 "1"
        map.insert(
            "p1".to_string(),
            PlayerData {
                id: "p1".to_string(),
                money: 100,
                xp: XPData {
                    current: 0,
                    required: 2,
                },
            },
        );
    
        Self {
            players: Arc::new(Mutex::new(map)),
        }
    }
    

    pub fn get_player(&self, player_id: &str) -> Option<PlayerData> {
        let players = self.players.lock().unwrap();
        players.get(player_id).cloned()
    }

    pub fn create_player(&self, player_id: &str) -> PlayerData {
        let mut players = self.players.lock().unwrap();
        let player_data = PlayerData {
            id: player_id.to_string(),
            money: 0,
            xp: XPData {
                current: 0,
                required: 2,
            },
        };
        players.insert(player_id.to_string(), player_data.clone());
        player_data
    }

    pub fn update_player(&self, player_data: PlayerData) {
        let mut players = self.players.lock().unwrap();
        players.insert(player_data.id.clone(), player_data);
    }

    pub fn buy_xp(&self, player_id: &str) -> Result<PlayerData, String> {
        let mut players = self.players.lock().unwrap();
        let player = players.get_mut(player_id).ok_or("Player not found")?;
        
        // 检查是否有足够的金钱
        if player.money < 4 {
            return Err("not enough money".to_string());
        }

        // 扣除金钱并增加经验值
        player.money -= 4;
        player.xp.current += 1;

        // 检查是否需要升级
        if player.xp.current >= player.xp.required {
            player.xp.current = 0;
            player.xp.required = (player.xp.required as f32 * 1.5).ceil() as i32;
        }

        Ok(player.clone())
    }
} 