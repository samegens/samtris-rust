use crate::high_scores::HighScore;

const MAX_NR_HIGH_SCORES: usize = 10;

#[derive(Debug, Clone)]
pub struct HighScores {
    scores: Vec<HighScore>,
}

impl HighScores {
    pub fn new() -> Self {
        Self { scores: Vec::new() }
    }

    pub fn from_vec(mut scores: Vec<HighScore>) -> Self {
        scores.sort_by(|a, b| b.score.cmp(&a.score));
        scores.truncate(MAX_NR_HIGH_SCORES);
        Self { scores }
    }

    pub fn is_high_score(&self, score: u32) -> bool {
        self.scores.len() < MAX_NR_HIGH_SCORES || score > self.scores.last().unwrap().score
    }

    pub fn add(&mut self, high_score: HighScore) -> bool {
        if !self.is_high_score(high_score.score) {
            return false;
        }

        self.scores.push(high_score);
        self.scores.sort_by(|a, b| b.score.cmp(&a.score));
        self.scores.truncate(MAX_NR_HIGH_SCORES);
        true
    }

    pub fn get_scores(&self) -> &[HighScore] {
        &self.scores
    }

    pub fn len(&self) -> usize {
        self.scores.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_high_scores() {
        // Act
        let sut = HighScores::new();

        // Assert
        assert_eq!(sut.len(), 0);
    }

    #[test]
    fn is_high_score_returns_true_when_list_not_full() {
        // Arrange
        let sut = HighScores::new();

        // Act & Assert
        assert!(sut.is_high_score(100));
    }

    #[test]
    fn is_high_score_returns_false_when_score_too_low() {
        // Arrange
        let scores = (1..=MAX_NR_HIGH_SCORES as u32)
            .map(|i| HighScore::new(format!("P{i}"), i * 1000, 1))
            .collect();
        let sut = HighScores::from_vec(scores);

        // Act
        let result: bool = sut.is_high_score(500);

        // Assert
        assert!(!result);
    }

    #[test]
    fn is_high_score_returns_true_when_score_high_enough() {
        // Arrange
        let scores = (1..=MAX_NR_HIGH_SCORES as u32)
            .map(|i| HighScore::new(format!("P{}", i), i * 1000, 1))
            .collect();
        let sut = HighScores::from_vec(scores);

        // Act
        let result: bool = sut.is_high_score(5500);

        // Assert
        assert!(result);
    }

    #[test]
    fn add_returns_true_and_adds_valid_high_score() {
        // Arrange
        let mut sut = HighScores::new();
        let new_score = HighScore::new("SAM".to_string(), 1000, 1);

        // Act
        let result = sut.add(new_score.clone());

        // Assert
        assert!(result);
        assert_eq!(sut.len(), 1);
        assert_eq!(sut.get_scores()[0], new_score);
    }

    #[test]
    fn add_returns_false_for_invalid_high_score() {
        // Arrange
        let scores = (1..=MAX_NR_HIGH_SCORES as u32)
            .map(|i| HighScore::new(format!("P{i}"), i * 1000, 1))
            .collect();
        let mut sut = HighScores::from_vec(scores);
        let low_score = HighScore::new("LOW".to_string(), 500, 1);

        // Act
        let result = sut.add(low_score);

        // Assert
        assert!(!result);
        assert_eq!(sut.len(), MAX_NR_HIGH_SCORES);
    }

    #[test]
    fn from_vec_sorts_scores_by_score_descending() {
        // Arrange
        let scores = vec![
            HighScore::new("P1".to_string(), 1000, 1),
            HighScore::new("P2".to_string(), 3000, 1),
            HighScore::new("P3".to_string(), 2000, 1),
        ];

        // Act
        let sut = HighScores::from_vec(scores);

        // Assert
        let scores = sut.get_scores();
        assert_eq!(scores[0].score, 3000);
        assert_eq!(scores[1].score, 2000);
        assert_eq!(scores[2].score, 1000);
    }
}
