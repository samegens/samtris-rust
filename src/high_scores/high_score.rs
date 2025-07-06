use crate::game_logic::GameResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighScore {
    pub name: String,
    pub game_result: GameResult,
}

impl HighScore {
    pub fn new(name: String, game_result: GameResult) -> Self {
        Self { name, game_result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_high_score_with_correct_values() {
        // Act
        let game_result = GameResult {
            score: 1000,
            level: 5,
        };
        let sut = HighScore::new("SAM".to_string(), game_result);

        // Assert
        assert_eq!(sut.name, "SAM");
        assert_eq!(sut.game_result.score, 1000);
        assert_eq!(sut.game_result.level, 5);
    }
}
