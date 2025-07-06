/// Platform-independent input event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    KeyPressed(Key),
    Quit,
}

/// Platform-independent key representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Space,
    Enter,
    Escape,
    Alphanumeric(char),
}
