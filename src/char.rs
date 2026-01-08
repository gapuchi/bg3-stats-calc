use crate::weapon::Weapon;

pub struct Character {
    stats: CharacterStats,
    weapon: Option<Weapon>,
}

impl Character {
    pub fn new() -> Self {
        Character {
            stats: CharacterStats {
                level: 1,
                strength: 16,
                dexterity: 8,
            },
            weapon: None,
        }
    }

    pub fn equip_weapon(&mut self, weapon: Weapon) {
        self.weapon = Some(weapon);
    }

    pub fn unequip_weapon(&mut self) {
        self.weapon = None;
    }

    pub fn attack(&self) {
        if let Some(weapon) = &self.weapon {
            weapon.attack_roll(self.stats);
            weapon.damage_roll(self.stats);
        }
    }
}

#[derive(Clone, Copy)]
pub struct CharacterStats {
    pub level: i32,
    pub strength: i32,
    pub dexterity: i32,
}
