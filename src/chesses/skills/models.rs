use serde::{Serialize, Deserialize};

/// 技能整體結構
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub skill_type: SkillType,
    pub trigger_condition: Option<TriggerCondition>,
    pub skill_effect: Vec<SkillEffectMeta>,
}

/// 技能類型
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SkillType {
    Active,
    Trigger,
}

/// 觸發條件
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerCondition {
    Always,
    OnHit,
    OnKill,
    OnAllyDeath,
    OnHpBelow { percent: u8 },
}

/// 效果包裝（含執行順序）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillEffectMeta {
    pub order: u8,
    pub effect: SkillEffect,
}

/// 技能效果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "PascalCase")]
pub enum SkillEffect {
    PhysicalDamage  { amount: u32, target: SkillTarget },
    MagicalDamage  { amount: u32, target: SkillTarget },
    TrueDamage     { amount: u32, target: SkillTarget },
    Heal           { amount: u32, target: SkillTarget },
    Dash          { distance: u32, target: SkillTarget },
    Buff    { effect: StatusEffect, target: SkillTarget },
    Debuff  { effect: StatusEffect, target: SkillTarget },
}

/// 目標對象
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SkillTarget {
    SelfTarget,
    SingleEnemy,
    AllEnemies,
    SingleAlly,
    AllAllies,
    RandomEnemy,
    RandomAlly,
    AreaOfEffect(AoeShape),
    Custom(String),
}

/// 範圍形狀
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AoeShape {
    Circle { radius: u32 },
    Line   { distance: u32 },
}

/// 狀態效果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusEffect {
    pub kind: StatusEffectType,
    pub amount: Option<i32>, 
    pub duration: u32,
}

/// 狀態類型
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum StatusEffectType {
    /* Buff */
    AttackDamageUp, 
    AbilityPowerUp,
    AttackSpeedUp,
    ArmorUp,
    MagicResistUp,
    Shield,
    /* Debuff */
    AttackDamageDown,
    AbilityPowerDown,
    AttackSpeedDown,
    ArmorDown,
    MagicResistDown,
    Stun,
}
