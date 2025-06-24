#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameInput {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateClockwise,
    RotateCounterclockwise,
    Drop,
    StartGame,
}
