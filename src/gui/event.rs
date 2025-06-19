#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Quit,
    GameInput(crate::gui::game_input::GameInput),
}
