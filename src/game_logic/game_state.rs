#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Playing,
    GameOver,
    ReturnToMainMenu,
    EnterHighScore(u32, u32), // Level, Score
}
