use crate::{ability::AbilityModifier, char::CharacterStats, die::Die};

pub trait Attack {
    fn roll(
        &self,
        character_stats: CharacterStats,
        ability_modifier: AbilityModifier,
    ) -> (i32, String);
}

pub struct BaseAttack {
    pub die: Die,
}

impl Attack for BaseAttack {
    fn roll(&self, _: CharacterStats, _: AbilityModifier) -> (i32, String) {
        let base = self.die.roll();
        (
            base,
            format!("{} ({}d{})", base, self.die.count, self.die.sides,),
        )
    }
}

pub struct AttackAbilityModifier {
    pub inner: Box<dyn Attack>,
}

impl Attack for AttackAbilityModifier {
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

pub struct ProficiencyModifier {
    pub inner: Box<dyn Attack>,
}

impl Attack for ProficiencyModifier {
    fn roll(&self, stats: CharacterStats, ability_modifier: AbilityModifier) -> (i32, String) {
        let (inner_val, inner_str) = self.inner.roll(stats, ability_modifier);

        let val = match stats.level {
            1..=4 => 2,
            5..=8 => 3,
            9..=12 => 4,
            _ => 0,
        };

        (
            inner_val + val,
            format!("{} + {} (Proficiency)", inner_str, val,),
        )
    }
}
