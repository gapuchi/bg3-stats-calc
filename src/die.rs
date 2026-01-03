use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Die {
    pub count: i32,
    pub sides: i32,
}

impl Die {
    pub fn new(sides: i32) -> Self {
        Die { count: 1, sides }
    }

    pub fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        (0..self.count).map(|_| rng.gen_range(1..=self.sides)).sum()
    }
}
