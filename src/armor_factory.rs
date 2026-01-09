use crate::{
    armor::Armor,
    armor_class::{ArmorClassIncrease, BaseArmorClass},
};

pub struct ArmorFactory {}

impl ArmorFactory {
    pub fn none() -> Armor {
        Armor {
            armor_class: Box::new(BaseArmorClass { armor_class: 10 }),
        }
    }

    pub fn wonderous_gloves() -> Armor {
        Armor {
            armor_class: Box::new(ArmorClassIncrease {
                delta: 1,
                inner: Box::new(BaseArmorClass { armor_class: 10 }),
            }),
        }
    }
}
