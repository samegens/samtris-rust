use crate::constants::*;
use crate::graphics::Display;
use crate::graphics::TetrominoPattern;
use crate::graphics::{I, J, L, O, S, T, Z};

pub struct MenuTitle {
    pattern: TetrominoPattern,
}

//TODO remove once use from main
#[allow(dead_code)]
impl MenuTitle {
    pub fn new() -> Self {
        // Pattern for "SAMTris" using readable letter constants
        let pattern_data = vec![
            vec![
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, T, T, T, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1,
            ],
            vec![
                -1, L, L, L, -1, J, J, -1, -1, O, O, -1, L, L, -1, -1, T, -1, -1, J, J, J, -1, -1,
                -1, I, I, I, I,
            ],
            vec![
                -1, L, -1, -1, -1, J, -1, I, -1, O, O, -1, Z, L, -1, -1, I, -1, -1, I, -1, J, -1,
                I, -1, J, -1, -1, -1,
            ],
            vec![
                -1, J, J, J, -1, J, Z, I, -1, J, J, Z, Z, L, -1, -1, I, -1, -1, I, S, -1, -1, I,
                -1, J, J, J, -1,
            ],
            vec![
                -1, -1, -1, J, -1, Z, Z, I, -1, J, -1, Z, -1, O, O, -1, I, -1, -1, I, S, S, -1, I,
                -1, -1, -1, L, -1,
            ],
            vec![
                I, I, I, I, -1, Z, -1, I, -1, J, -1, -1, -1, O, O, -1, I, -1, -1, I, -1, S, -1, I,
                -1, L, L, L, -1,
            ],
            vec![
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1, -1, -1, -1, -1,
            ],
        ];

        Self {
            pattern: TetrominoPattern::new(pattern_data),
        }
    }

    pub fn draw<D: Display + ?Sized>(&self, display: &mut D) -> Result<(), String> {
        let y = WINDOW_HEIGHT_IN_BLOCKS * BLOCK_SIZE / 6;
        self.pattern.draw_centered(display, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::MockDisplay;

    #[test]
    fn menu_title_draws_blocks() {
        // Arrange
        let sut = MenuTitle::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(!display.drawn_blocks.is_empty());
    }

    #[test]
    fn menu_title_uses_different_tetromino_types() {
        // Arrange
        let sut = MenuTitle::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());

        let used_types: std::collections::HashSet<_> = display
            .drawn_blocks
            .iter()
            .map(|(_, tetromino_type)| *tetromino_type)
            .collect();

        assert!(used_types.len() > 1, "Should use multiple tetromino types");
    }
}
