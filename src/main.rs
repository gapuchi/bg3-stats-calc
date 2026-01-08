use crate::{char::Character, weapon_factory::WeaponFactory};

mod ability;
mod attack;
mod char;
mod damage;
mod die;
mod weapon;
mod weapon_factory;

fn main() {
    let mut char = Character::new();

    char.equip_weapon(WeaponFactory::halberd());

    char.attack();
    char.attack();

    char.equip_weapon(WeaponFactory::shortbow());

    char.attack();
    char.attack();

    char.unequip_weapon();
}
