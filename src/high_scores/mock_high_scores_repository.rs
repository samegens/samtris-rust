use std::cell::RefCell;

use crate::high_scores::{HighScores, HighScoresRepository};

pub struct MockHighScoresRepository {
    high_scores: RefCell<HighScores>,
    pub fail_on_save: bool,
}

impl MockHighScoresRepository {
    pub fn new(high_scores: HighScores) -> Self {
        Self {
            high_scores: RefCell::new(high_scores),
            fail_on_save: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            high_scores: RefCell::new(HighScores::new()),
            fail_on_save: false,
        }
    }
}

impl HighScoresRepository for MockHighScoresRepository {
    fn load(&self) -> Result<HighScores, String> {
        Ok(self.high_scores.borrow().clone())
    }

    fn save(&self, high_scores: &HighScores) -> Result<(), String> {
        if self.fail_on_save {
            return Err("Mock save failure".to_string());
        }

        *self.high_scores.borrow_mut() = high_scores.clone();
        Ok(())
    }
}
