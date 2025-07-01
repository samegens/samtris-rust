// src/high_scores/manager.rs
use crate::high_scores::{HighScore, HighScores, HighScoresRepository};

pub struct HighScoreManager {
    repository: Box<dyn HighScoresRepository>,
    high_scores: HighScores,
}

impl HighScoreManager {
    pub fn new(repository: Box<dyn HighScoresRepository>) -> Self {
        let high_scores = repository.load().expect("Failed to load high scores");
        Self {
            repository,
            high_scores,
        }
    }

    pub fn is_high_score(&self, score: u32) -> bool {
        self.high_scores.is_high_score(score)
    }

    pub fn add_high_score(&mut self, high_score: HighScore) -> Result<bool, String> {
        let was_added = self.high_scores.add(high_score);
        if was_added {
            self.repository.save(&self.high_scores)?;
        }
        Ok(was_added)
    }

    pub fn get_high_scores(&self) -> &HighScores {
        &self.high_scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::high_scores::MockHighScoresRepository;

    #[test]
    fn new_loads_high_scores_from_repository() {
        // Arrange
        let mut initial_scores = HighScores::new();
        initial_scores.add(HighScore::new("SAM".to_string(), 1000, 5));
        let repository = Box::new(MockHighScoresRepository::new(initial_scores));

        // Act
        let result = HighScoreManager::new(repository);

        // Assert
        assert_eq!(result.get_high_scores().len(), 1);
    }

    #[test]
    fn is_high_score_delegates_to_high_scores() {
        // Arrange
        let repository = Box::new(MockHighScoresRepository::empty());
        let sut = HighScoreManager::new(repository);

        // Act & Assert
        assert!(sut.is_high_score(1)); // Empty list, any score qualifies
    }

    #[test]
    fn add_high_score_saves_when_score_qualifies() {
        // Arrange
        let repository = Box::new(MockHighScoresRepository::empty());
        let mut sut = HighScoreManager::new(repository);
        let new_score = HighScore::new("TEST".to_string(), 500, 2);

        // Act
        let result = sut.add_high_score(new_score.clone());

        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap()); // Score was added
        assert_eq!(sut.get_high_scores().len(), 1);
    }

    #[test]
    fn add_high_score_does_not_save_when_score_does_not_qualify() {
        // Arrange
        let mut initial_scores = HighScores::new();
        for i in 1..=10 {
            initial_scores.add(HighScore::new(format!("P{i}"), i * 1000, 1));
        }
        let repository = Box::new(MockHighScoresRepository::new(initial_scores));
        let mut sut = HighScoreManager::new(repository);
        let low_score = HighScore::new("LOW".to_string(), 500, 1);

        // Act
        let result = sut.add_high_score(low_score);

        // Assert
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Score was not added
        assert_eq!(sut.get_high_scores().len(), 10); // Unchanged
    }
}
