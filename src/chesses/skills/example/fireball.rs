use crate::chesses::skills::models::{
    Skill, SkillType, SkillEffect, SkillEffectMeta, SkillTarget, AttrType, StatusEffect, StatusEffectType,
};

pub fn fireball() -> Skill {
    Skill {
        id: "fireball".into(),
        name: "Fireball".into(),
        description: "Hurl a fireball at a single enemy.".into(),
        skill_type: SkillType::Active,
        trigger_condition: None,
        skill_effect: vec![
            SkillEffectMeta { 
                order: 1, 
                effect: SkillEffect::MagicalDamage { 
                    attr: AttrType::AbilityPower,
                    ratio: 1.5,
                    target: SkillTarget::SingleEnemy 
                } 
            },
            SkillEffectMeta { 
                order: 2, 
                effect: SkillEffect::Debuff {
                    effect: StatusEffect { 
                        kind: StatusEffectType::Stun, 
                        amount: None, 
                        duration: 3 
                    },
                    target: SkillTarget::SingleEnemy
                } 
            },
        ],
    }
}
