use crate::{ability::get_modifier, char::CharacterStats};

pub trait ArmorClass {
    fn get(&self, stats: CharacterStats) -> (i32, String);
}

pub struct BaseArmorClass {
    pub armor_class: i32,
}

impl ArmorClass for BaseArmorClass {
    fn get(&self, stats: CharacterStats) -> (i32, String) {
        let modifier = get_modifier(stats.dexterity);
        let val = self.armor_class + modifier;

        (val, format!("10 + {} (Dex Modifier)", modifier))
    }
}

pub struct ArmorClassIncrease {
    pub inner: Box<dyn ArmorClass>,
    pub delta: i32,
}

impl ArmorClass for ArmorClassIncrease {
    fn get(&self, stats: CharacterStats) -> (i32, String) {
        let (val, description) = self.inner.get(stats);

        (
            val + self.delta,
            format!("{} + {}", description, self.delta),
        )
    }
}
