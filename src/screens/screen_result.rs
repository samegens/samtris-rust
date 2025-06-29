#[derive(Debug, PartialEq, Eq)]
pub enum ScreenResult {
    Continue,
    ReturnToMainMenu,
    Play,
    ShowHighScores,
    EnterHighScore,
    Quit,
}
