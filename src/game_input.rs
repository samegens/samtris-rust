// TODO: remove suppressions once more enum values are used.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameInput {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateClockwise,
    RotateCounterclockwise,
    // Drop,
    // Quit,
}
