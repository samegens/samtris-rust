use crate::high_scores::HighScore;

pub const MAX_NR_HIGH_SCORES: usize = 10;

#[derive(Debug, Clone)]
pub struct HighScores {
    scores: Vec<HighScore>,
}

impl HighScores {
    pub fn new() -> Self {
        Self { scores: Vec::new() }
    }

    pub fn from_vec(mut scores: Vec<HighScore>) -> Self {
        scores.sort_by(|a, b| b.game_result.score.cmp(&a.game_result.score));
        scores.truncate(MAX_NR_HIGH_SCORES);
        Self { scores }
    }

    pub fn is_high_score(&self, score: u32) -> bool {
        self.scores.len() < MAX_NR_HIGH_SCORES
            || score > self.scores.last().unwrap().game_result.score
    }

    /// Adds a new high score if it qualifies as a high score. Returns the index of high_score
    /// was indeed a high score, otherwise MAX_NR_HIGH_SCORES.
    pub fn add(&mut self, high_score: HighScore) -> usize {
        if !self.is_high_score(high_score.game_result.score) {
            return MAX_NR_HIGH_SCORES;
        }

        self.scores.push(high_score.clone());
        self.scores
            .sort_by(|a, b| b.game_result.score.cmp(&a.game_result.score));
        self.scores.truncate(MAX_NR_HIGH_SCORES);

        if let Some(pos) = self.scores.iter().position(|s| s == &high_score) {
            pos
        } else {
            MAX_NR_HIGH_SCORES
        }
    }

    pub fn get_scores(&self) -> &[HighScore] {
        &self.scores
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.scores.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::game_logic::GameResult;

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
            .map(|i| {
                HighScore::new(
                    format!("P{i}"),
                    GameResult {
                        score: i * 1000,
                        level: 1,
                    },
                )
            })
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
            .map(|i| {
                HighScore::new(
                    format!("P{i}"),
                    GameResult {
                        score: i * 1000,
                        level: 1,
                    },
                )
            })
            .collect();
        let sut = HighScores::from_vec(scores);

        // Act
        let result: bool = sut.is_high_score(5500);

        // Assert
        assert!(result);
    }

    #[test]
    fn add_returns_0_when_no_highscores() {
        // Arrange
        let mut sut = HighScores::new();
        let new_score = HighScore::new(
            "SAM".to_string(),
            GameResult {
                score: 1000,
                level: 1,
            },
        );

        // Act
        let result = sut.add(new_score.clone());

        // Assert
        assert_eq!(result, 0);
        assert_eq!(sut.len(), 1);
        assert_eq!(sut.get_scores()[0], new_score);
    }

    #[test]
    fn add_returns_correct_index_when_highscores_present() {
        // Arrange
        let mut sut = HighScores::new();
        for i in 1..=10 {
            let score = HighScore::new(
                format!("P{i}"),
                GameResult {
                    score: i * 100,
                    level: 1,
                },
            );
            sut.add(score);
        }
        let new_score = HighScore::new(
            "SAM".to_string(),
            GameResult {
                score: 550,
                level: 1,
            },
        );

        // Act
        let result = sut.add(new_score.clone());

        // Assert
        assert_eq!(result, 5);
        assert_eq!(sut.len(), 10);
        assert_eq!(sut.get_scores()[5], new_score);
    }

    #[test]
    fn add_returns_max_for_invalid_high_score() {
        // Arrange
        let scores = (1..=MAX_NR_HIGH_SCORES as u32)
            .map(|i| {
                HighScore::new(
                    format!("P{i}"),
                    GameResult {
                        score: i * 1000,
                        level: 1,
                    },
                )
            })
            .collect();
        let mut sut = HighScores::from_vec(scores);
        let low_score = HighScore::new(
            "LOW".to_string(),
            GameResult {
                score: 500,
                level: 1,
            },
        );

        // Act
        let result = sut.add(low_score);

        // Assert
        assert_eq!(result, MAX_NR_HIGH_SCORES);
        assert_eq!(sut.len(), MAX_NR_HIGH_SCORES);
    }

    #[test]
    fn from_vec_sorts_scores_by_score_descending() {
        // Arrange
        let scores = vec![
            HighScore::new(
                "P1".to_string(),
                GameResult {
                    score: 1000,
                    level: 1,
                },
            ),
            HighScore::new(
                "P2".to_string(),
                GameResult {
                    score: 3000,
                    level: 1,
                },
            ),
            HighScore::new(
                "P3".to_string(),
                GameResult {
                    score: 2000,
                    level: 1,
                },
            ),
        ];

        // Act
        let sut = HighScores::from_vec(scores);

        // Assert
        let scores = sut.get_scores();
        assert_eq!(scores[0].game_result.score, 3000);
        assert_eq!(scores[1].game_result.score, 2000);
        assert_eq!(scores[2].game_result.score, 1000);
    }
}
