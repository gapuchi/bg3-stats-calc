use crate::{armor::Armor, armor_factory::ArmorFactory, weapon::Weapon};

pub struct Character {
    armor: Option<Armor>,
    stats: CharacterStats,
    weapon_slot_1: Option<Weapon>,
}

impl Character {
    pub fn new() -> Self {
        Character {
            armor: None,
            stats: CharacterStats {
                level: 1,
                strength: 16,
                dexterity: 8,
            },
            weapon_slot_1: None,
        }
    }

    pub fn equip_armor(&mut self, armor: Armor) {
        self.armor = Some(armor);
    }

    pub fn equip_weapon(&mut self, weapon: Weapon) {
        self.weapon_slot_1 = Some(weapon);
    }

    pub fn unequip_weapon(&mut self) {
        self.weapon_slot_1 = None;
    }

    pub fn attack(&self, other: &mut Character) {
        if let Some(weapon) = &self.weapon_slot_1 {
            let attack_roll = weapon.attack_roll(self.stats);

            if attack_roll < other.armor_class() {
                println!(
                    "Attack Roll {}, AC {} Missed!",
                    attack_roll,
                    other.armor_class()
                );
                return;
            }

            weapon.damage_roll(self.stats);
        }
    }

    pub fn armor_class(&self) -> i32 {
        if let Some(armor) = &self.armor {
            armor.armor_class(self.stats)
        } else {
            ArmorFactory::none().armor_class(self.stats)
        }
    }
}

#[derive(Clone, Copy)]
pub struct CharacterStats {
    pub level: i32,
    pub strength: i32,
    pub dexterity: i32,
}
