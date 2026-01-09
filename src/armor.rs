use crate::{armor_class::ArmorClass, char::CharacterStats};

pub struct Armor {
    pub armor_class: Box<dyn ArmorClass>,
}

impl Armor {
    pub fn armor_class(&self, stats: CharacterStats) -> i32 {
        let (val, description) = self.armor_class.get(stats);
        println!("AC {} = {}", val, description);
        val
    }
}
