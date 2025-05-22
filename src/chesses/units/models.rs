use serde::{Serialize, Deserialize};
use crate::chesses::skills::models::Skill;
use crate::chesses::skills::models::StatusEffect;

/// 英雄的主要資料結構
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChessTemplate {
    pub id: String,                         // e.g., "garen_1" , rule: name_star
    pub chess: String,                       // 顯示名稱
    pub level: StarLevel,
    pub description: Option<String>,        // 英雄簡介（可選）
    pub base_attrs: Attrs,                  // 英雄基礎屬性
    pub skills: Vec<Skill>,                 // 英雄所擁有的技能列表
    pub synergies: Vec<SynergyTag>,         // 英雄所屬的羈絆標籤
}

/// 英雄的數值屬性
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attrs {
    pub max_hp: i32,
    pub max_mp: i32,
    pub armor: i32,
    pub magic_resist: i32,
    pub attack_damage: i32,
    pub ability_power: i32,
    pub attack_speed: f32,
    pub attack_range: i32,
}

/// 羈絆系統標籤（Trait / Origin / Class）
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SynergyTag {
    Knight,
    Mage,
    Assassin,
    Human,
    Orc,
    Undead,
    // …其他羈絆
}

/// 星級（棋子等階）
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum StarLevel {
    One,
    Two,
    Three,
}



/* For runtime usage */

/// 戰鬥中單位實例
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    /// 參考的靜態模板
    pub template: ChessTemplate,
    /// 戰鬥中可變的屬性狀態
    pub state: UnitState,
    /// 當前持有的狀態效果
    pub status_effects: Vec<StatusEffect>,
}

/// 戰鬥中可變的屬性
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitState {
    pub hp: i32,
    pub mp: i32,
    pub attack_damage: i32,
    pub ability_power: i32,
    pub armor: i32,
    pub magic_resist: i32,
    pub attack_speed: i32,
}



