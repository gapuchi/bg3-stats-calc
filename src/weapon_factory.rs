use crate::{
    ability::AbilityModifier,
    damage::{AddAbilityModifier, BaseDamage, DamageType},
    die::Die,
    weapon::Weapon,
};

pub struct WeaponFactory {}

impl WeaponFactory {
    pub fn halberd() -> Weapon {
        Weapon {
            ability_modifier: AbilityModifier::Strength,
            damage: Box::new(AddAbilityModifier {
                inner: Box::new(BaseDamage {
                    die: Die::new(10),
                    damage_type: DamageType::Slashing,
                }),
            }),
        }
    }

    pub fn shortbow() -> Weapon {
        Weapon {
            ability_modifier: AbilityModifier::Dexterity,
            damage: Box::new(AddAbilityModifier {
                inner: Box::new(BaseDamage {
                    die: Die::new(6),
                    damage_type: DamageType::Piercing,
                }),
            }),
        }
    }
}
