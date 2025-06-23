#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    LevelStarted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    LevelStarted(u32),
}
