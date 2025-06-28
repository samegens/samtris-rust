use crate::tetromino::TetrominoType;

#[derive(Debug, Clone, PartialEq)]
pub struct HudView {
    pub next_tetromino_type: TetrominoType,
    pub current_level: u32,
    pub total_lines_cleared: u32,
    pub show_game_over: bool,
}
