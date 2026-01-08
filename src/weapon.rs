use colored::Colorize;

use crate::{ability::AbilityModifier, attack::Attack, char::CharacterStats, damage::Damage};

pub struct Weapon {
    pub ability_modifier: AbilityModifier,
    pub attack: Box<dyn Attack>,
    pub damage: Box<dyn Damage>,
}

impl Weapon {
    pub fn damage_roll(&self, character_stats: CharacterStats) -> i32 {
        let (damage, descr) = self.damage.roll(character_stats, self.ability_modifier);

        println!("\n{}", "=== Damage Roll ===".bold().bright_yellow());
        println!("{} = {}", damage, descr);

        damage
    }

    pub fn attack_roll(&self, character_stats: CharacterStats) -> i32 {
        let (attack, descr) = self.attack.roll(character_stats, self.ability_modifier);

        println!("\n{}", "=== Attack Roll ===".bold().bright_yellow());
        println!("{} = {}", attack, descr);

        attack
    }
}
