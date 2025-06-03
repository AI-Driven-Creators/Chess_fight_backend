use crate::types::game_state::ChessPiece;

pub fn all_chess_pieces() -> Vec<ChessPiece> {
    vec![
        ChessPiece { name: "Mage".into(), cost: 3, level: 1 },
        ChessPiece { name: "Knight".into(), cost: 2, level: 1 },
        ChessPiece { name: "Assassin".into(), cost: 4, level: 1 },
		ChessPiece { name: "Mage".into(), cost: 3, level: 1 },
		ChessPiece { name: "Tank".into(), cost: 3, level: 1 },
		ChessPiece { name: "Knight".into(), cost: 3, level: 1 },
		ChessPiece { name: "Priest".into(), cost: 3, level: 1 },
		ChessPiece { name: "Hunter".into(), cost: 3, level: 1 },
		ChessPiece { name: "Assassin".into(), cost: 3, level: 1 },
		ChessPiece { name: "Archer".into(), cost: 3, level: 1 },
		ChessPiece { name: "Berserker".into(), cost: 3, level: 1 },
		ChessPiece { name: "Paladin".into(), cost: 3, level: 1 },
		ChessPiece { name: "Warlock".into(), cost: 3, level: 1 },
		ChessPiece { name: "Necromancer".into(), cost: 3, level: 1 },
		ChessPiece { name: "Druid".into(), cost: 3, level: 1 },
		ChessPiece { name: "Shaman".into(), cost: 3, level: 1 },
		ChessPiece { name: "Blademaster".into(), cost: 3, level: 1 },
		ChessPiece { name: "Sniper".into(), cost: 3, level: 1 },
		ChessPiece { name: "Engineer".into(), cost: 3, level: 1 },
		ChessPiece { name: "Beastmaster".into(), cost: 3, level: 1 },
		ChessPiece { name: "Phantom".into(), cost: 3, level: 1 },
		ChessPiece { name: "Guardian".into(), cost: 3, level: 1 },
		ChessPiece { name: "Elemental".into(), cost: 3, level: 1 }
	       // ...更多棋子
    ]
}

pub fn initial_money() -> u32 { 100 }
pub fn initial_experience() -> u32 { 0 }