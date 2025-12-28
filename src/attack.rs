use rand::Rng;

/// The base trait for all attack rolls.
pub trait AttackRoll {
    /// Performs the roll calculation.
    fn roll(&self) -> i32;
    /// Provides a human-readable description of how the roll is calculated.
    fn description(&self) -> String;
}

/// The base implementation of an attack roll (a simple d20).
pub struct DieRoll {
    pub name: String,
}

impl DieRoll {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl AttackRoll for DieRoll {
    fn roll(&self) -> i32 {
        // Return a random value between 1 and 20.
        rand::thread_rng().gen_range(1..=20)
    }

    fn description(&self) -> String {
        format!("1d20 ({})", self.name)
    }
}

/// A decorator that adds a Strength-based modifier to an attack roll.
pub struct StrengthDecorator<T: AttackRoll> {
    pub inner: T,
    pub strength_score: i32,
}

impl<T: AttackRoll> AttackRoll for StrengthDecorator<T> {
    fn roll(&self) -> i32 {
        let modifier = (self.strength_score - 10) / 2;
        self.inner.roll() + modifier
    }

    fn description(&self) -> String {
        let modifier = (self.strength_score - 10) / 2;
        format!("{} + {} (STR)", self.inner.description(), modifier)
    }
}

/// A decorator that adds a Dexterity-based modifier to an attack roll.
pub struct DexterityDecorator<T: AttackRoll> {
    pub inner: T,
    pub dexterity_score: i32,
}

impl<T: AttackRoll> AttackRoll for DexterityDecorator<T> {
    fn roll(&self) -> i32 {
        let modifier = (self.dexterity_score - 10) / 2;
        self.inner.roll() + modifier
    }

    fn description(&self) -> String {
        let modifier = (self.dexterity_score - 10) / 2;
        format!("{} + {} (DEX)", self.inner.description(), modifier)
    }
}

/// A decorator that adds a Proficiency bonus.
pub struct ProficiencyDecorator<T: AttackRoll> {
    pub inner: T,
    pub bonus: i32,
}

impl<T: AttackRoll> AttackRoll for ProficiencyDecorator<T> {
    fn roll(&self) -> i32 {
        self.inner.roll() + self.bonus
    }

    fn description(&self) -> String {
        format!(
            "{} + {} (Proficiency)",
            self.inner.description(),
            self.bonus
        )
    }
}
