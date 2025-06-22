use crate::{graphics::Display, playfield::Playfield, tetromino::TetrominoInstance};

pub trait PlayfieldRenderer {
    fn draw<D: Display>(
        &self,
        playfield: &Playfield,
        current_tetromino: Option<&TetrominoInstance>,
        blinking_lines: &[u32],
        show_blinking_lines: bool,
        display: &mut D,
    ) -> Result<(), String>;
}
