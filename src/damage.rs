use crate::{ability::AbilityModifier, char::CharacterStats, die::Die};

#[derive(Debug)]
pub enum DamageType {
    Slashing,
    Piercing,
}

pub trait Damage {
    fn roll(
        &self,
        character_stats: CharacterStats,
        ability_modifier: AbilityModifier,
    ) -> (i32, String);
}

pub struct BaseDamage {
    pub die: Die,
    pub damage_type: DamageType,
}

impl Damage for BaseDamage {
    fn roll(&self, _: CharacterStats, _: AbilityModifier) -> (i32, String) {
        let base = self.die.roll();
        (
            base,
            format!(
                "{} ({}d{} {:#?})",
                base, self.die.count, self.die.sides, self.damage_type
            ),
        )
    }
}

pub struct AddAbilityModifier {
    pub inner: Box<dyn Damage>,
}

impl Damage for AddAbilityModifier {
    fn roll(&self, stats: CharacterStats, ability_modifier: AbilityModifier) -> (i32, String) {
        let (inner_val, inner_str) = self.inner.roll(stats, ability_modifier);

        let val = match ability_modifier {
            AbilityModifier::Strength => (stats.strength - 10) / 2,
            AbilityModifier::Dexterity => (stats.dexterity - 10) / 2,
        };

        (
            inner_val + val,
            format!("{} + {} ({} Modifier)", inner_str, val, ability_modifier),
        )
    }
}
