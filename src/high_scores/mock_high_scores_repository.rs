use std::cell::RefCell;

use crate::high_scores::{HighScores, HighScoresRepository};

pub struct MockHighScoresRepository {
    high_scores: RefCell<HighScores>,
}

impl MockHighScoresRepository {
    pub fn new(high_scores: HighScores) -> Self {
        Self {
            high_scores: RefCell::new(high_scores),
        }
    }

    pub fn empty() -> Self {
        Self {
            high_scores: RefCell::new(HighScores::new()),
        }
    }
}

impl HighScoresRepository for MockHighScoresRepository {
    fn load(&self) -> Result<HighScores, String> {
        Ok(self.high_scores.borrow().clone())
    }

    fn save(&self, high_scores: &HighScores) -> Result<(), String> {
        *self.high_scores.borrow_mut() = high_scores.clone();
        Ok(())
    }
}
