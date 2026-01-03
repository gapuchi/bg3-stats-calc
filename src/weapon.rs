use crate::{ability::AbilityModifier, char::CharacterStats, damage::Damage, die::Die};

pub struct Weapon {
    pub ability_modifier: AbilityModifier,
    pub damage: Box<dyn Damage>,
}

impl Weapon {
    pub fn damage_roll(&self, character_stats: CharacterStats) -> i32 {
        let (damage, descr) = self.damage.roll(character_stats, self.ability_modifier);

        println!("{} = {}", damage, descr);

        damage
    }

    pub fn attack_roll(&self, character_stats: CharacterStats) -> i32 {
        let base = Die::new(20).roll();

        let relevant_stat = match self.ability_modifier {
            AbilityModifier::Strength => character_stats.strength,
            AbilityModifier::Dexterity => character_stats.dexterity,
        };

        let modifier = (relevant_stat - 10) / 2;

        base + modifier
    }
}
