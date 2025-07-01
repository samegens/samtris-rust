use crate::high_scores::HighScores;

pub trait HighScoresRepository {
    fn load(&self) -> Result<HighScores, String>;
    fn save(&self, high_scores: &HighScores) -> Result<(), String>;
}
