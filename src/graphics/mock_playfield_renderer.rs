use std::cell::RefCell;

use crate::graphics::{Display, PlayfieldRenderer, PlayfieldView};

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
    fn draw<D: Display + ?Sized>(
        &self,
        playfield_view: &PlayfieldView,
        _display: &mut D,
    ) -> Result<(), String> {
        self.draw_calls.borrow_mut().push(DrawCall {
            blinking_lines: playfield_view.full_lines.to_vec(),
            show_blinking_lines: playfield_view.show_blinking_lines,
        });
        Ok(())
    }
}
