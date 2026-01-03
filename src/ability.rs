use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityModifier {
    Strength,
    Dexterity,
}

impl fmt::Display for AbilityModifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AbilityModifier::Strength => write!(f, "Strength"),
            AbilityModifier::Dexterity => write!(f, "Dexterity"),
        }
    }
}
