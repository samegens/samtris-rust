#[derive(Debug, PartialEq, Eq)]
pub enum ScreenResult {
    Continue,
    ReturnToMainMenu,
    Play,
    ShowHighScores,
    EnterHighScore(u32, u32), // Level, Score
    Quit,
}
