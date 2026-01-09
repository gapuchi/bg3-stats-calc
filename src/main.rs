use crate::{armor_factory::ArmorFactory, char::Character, weapon_factory::WeaponFactory};

mod ability;
mod armor;
mod armor_class;
mod armor_factory;
mod attack;
mod char;
mod damage;
mod die;
mod weapon;
mod weapon_factory;

fn main() {
    let mut char = Character::new();
    let mut enemy = Character::new();

    enemy.equip_armor(ArmorFactory::wonderous_gloves());

    char.equip_weapon(WeaponFactory::halberd());

    char.attack(&mut enemy);
    char.attack(&mut enemy);

    char.equip_weapon(WeaponFactory::shortbow());

    char.attack(&mut enemy);
    char.attack(&mut enemy);

    char.unequip_weapon();
}
