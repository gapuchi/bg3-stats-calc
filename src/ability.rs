use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityModifier {
    Strength,
    Dexterity,
}

impl fmt::Display for AbilityModifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AbilityModifier::Strength => write!(f, "Str"),
            AbilityModifier::Dexterity => write!(f, "Dex"),
        }
    }
}

pub fn get_modifier(stat: i32) -> i32 {
    (stat - 10) / 2
}
