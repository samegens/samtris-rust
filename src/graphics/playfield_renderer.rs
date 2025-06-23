use crate::graphics::{Display, PlayfieldView};
use crate::tetromino::TetrominoInstance;

pub trait PlayfieldRenderer {
    fn draw<D: Display>(
        &self,
        playfield_view: &PlayfieldView,
        current_tetromino: Option<&TetrominoInstance>,
        blinking_lines: &[u32],
        show_blinking_lines: bool,
        display: &mut D,
    ) -> Result<(), String>;
}
