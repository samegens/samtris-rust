#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    LevelStarted(u32), // Level number (0-based)
    LinesCleared(u32), // Number of lines cleared
}
