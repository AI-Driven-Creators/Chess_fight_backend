use crate::chesses::skills::models::*;
use once_cell::sync::Lazy;

pub static FIREBALL: Lazy<Skill> = Lazy::new(|| Skill {
    id: "fireball".into(),
    name: "Fireball".into(),
    description: "Hurl a fireball at a single enemy.".into(),
    skill_type: SkillType::Active,
    trigger_condition: None,
    skill_effect: vec![
        SkillEffectMeta { 
            order: 1, 
            effect: SkillEffect::MagicalDamage { 
                amount: 100, 
                target: SkillTarget::SingleEnemy 
            } 
        },
        SkillEffectMeta { 
            order: 2, 
            effect: SkillEffect::Debuff {
                effect: StatusEffect { 
                    kind: StatusEffectType::Stun, 
                    amount: Some(10), 
                    duration: 3 
                },
                target: SkillTarget::SingleEnemy
            } 
        },
    ],
});