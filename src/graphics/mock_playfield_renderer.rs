use std::cell::RefCell;

use crate::graphics::{Display, PlayfieldRenderer};
use crate::playfield::Playfield;
use crate::tetromino::TetrominoInstance;

/// MockPlayfieldRenderer is a mock implementation of PlayfieldRenderer for testing purposes.
#[derive(Debug, Default)]
pub struct MockPlayfieldRenderer {
    pub draw_calls: RefCell<Vec<DrawCall>>,
}

#[derive(Debug, Clone)]
pub struct DrawCall {
    pub blinking_lines: Vec<u32>,
    pub show_blinking_lines: bool,
}

impl MockPlayfieldRenderer {
    pub fn new() -> Self {
        Self {
            draw_calls: RefCell::new(Vec::new()),
        }
    }

    pub fn get_draw_calls(&self) -> Vec<DrawCall> {
        self.draw_calls.borrow().clone()
    }
}

impl PlayfieldRenderer for MockPlayfieldRenderer {
    fn draw<D: Display>(
        &self,
        _playfield: &Playfield,
        _current_tetromino: Option<&TetrominoInstance>,
        blinking_lines: &[u32],
        show_blinking_lines: bool,
        _display: &mut D,
    ) -> Result<(), String> {
        self.draw_calls.borrow_mut().push(DrawCall {
            blinking_lines: blinking_lines.to_vec(),
            show_blinking_lines,
        });
        Ok(())
    }
}
