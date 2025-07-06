use crate::game_logic::GameResult;

#[derive(Debug, PartialEq, Eq)]
pub enum ScreenResult {
    Continue,
    ReturnToMainMenu,
    Play,
    ShowHighScores,
    EnterHighScore(GameResult),
    Quit,
}
