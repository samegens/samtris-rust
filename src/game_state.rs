#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    Playing,
    GameOver,
    AnimatingLines {
        countdown: std::time::Duration,
    },
}
