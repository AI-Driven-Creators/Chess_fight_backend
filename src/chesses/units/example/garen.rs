use crate::chesses::units::models::{ ChessTemplate, StarLevel, Attrs, SynergyTag };
use crate::chesses::skills::example::fireball::fireball; // 改成函式

/// 範例：一星蓋倫（Garen）模板
/// 這是一個會用火球術的蓋倫
pub fn garen_1() -> ChessTemplate {
    ChessTemplate {
        id: "garen_1".into(),
        chess: "Garen".into(),
        level: StarLevel::One,
        description: Some("Demacia’s mightiest warrior, excels in melee combat.".into()),
        base_attrs: Attrs {
            max_hp: 700,
            max_mp: 100,
            armor: 50,
            magic_resist: 30,
            attack_damage: 75,
            ability_power: 0,
            attack_speed: 0.72,
            attack_range: 1,
        },
        skills: vec![
            fireball(), // 每次複製一份新 skill
        ],
        synergies: vec![SynergyTag::Knight, SynergyTag::Human],
    }
}
