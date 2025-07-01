mod high_score;
mod high_scores;
mod high_scores_repository;
#[cfg(test)]
mod mock_high_scores_repository;

pub use high_score::HighScore;
pub use high_scores::HighScores;
//TODO: Remove once used by main
#[allow(unused_imports)]
pub use high_scores_repository::HighScoresRepository;
//TODO: Remove once used by tests
#[allow(unused_imports)]
#[cfg(test)]
pub use mock_high_scores_repository::MockHighScoresRepository;
