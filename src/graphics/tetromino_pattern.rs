use crate::common::Position;
use crate::constants::*;
use crate::graphics::Display;
use crate::tetromino::TetrominoType;

// Constants for tetromino types - makes patterns much more readable
pub const I: i8 = 0;
pub const O: i8 = 1;
pub const T: i8 = 2;
pub const Z: i8 = 3;
pub const S: i8 = 4;
pub const J: i8 = 5;
pub const L: i8 = 6;

pub struct TetrominoPattern {
    pattern: Vec<Vec<i8>>,
}

impl TetrominoPattern {
    pub fn new(pattern: Vec<Vec<i8>>) -> Self {
        Self { pattern }
    }

    pub fn draw<D: Display + ?Sized>(&self, display: &mut D, x: u32, y: u32) -> Result<(), String> {
        for (row_index, row) in self.pattern.iter().enumerate() {
            for (col_index, &cell) in row.iter().enumerate() {
                if cell >= 0 {
                    let tetromino_type = self.number_to_tetromino_type(cell);
                    let block_x = x + (col_index as u32 * BLOCK_SIZE);
                    let block_y = y + (row_index as u32 * BLOCK_SIZE);
                    let pos = Position::new(block_x as i32, block_y as i32);
                    display.draw_block(pos, tetromino_type)?;
                }
            }
        }
        Ok(())
    }

    pub fn draw_centered<D: Display + ?Sized>(
        &self,
        display: &mut D,
        center_y: u32,
    ) -> Result<(), String> {
        let pattern_width = self.get_width() * BLOCK_SIZE;
        let center_x = (WINDOW_WIDTH_IN_BLOCKS * BLOCK_SIZE - pattern_width) / 2;
        self.draw(display, center_x, center_y)
    }

    pub fn get_width(&self) -> u32 {
        self.pattern
            .first()
            .map(|row| row.len() as u32)
            .unwrap_or(0)
    }

    fn number_to_tetromino_type(&self, number: i8) -> TetrominoType {
        match number {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::Z,
            4 => TetrominoType::S,
            5 => TetrominoType::J,
            6 => TetrominoType::L,
            _ => {
                panic!("Invalid tetromino number: {}", number);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::MockDisplay;
    use rstest::rstest;

    #[test]
    fn tetromino_pattern_draws_blocks() {
        // Arrange
        let sut = TetrominoPattern::new(create_test_pattern());
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display, 0, 0);

        // Assert
        assert!(result.is_ok());
        assert_eq!(display.drawn_blocks.len(), 4);
    }

    #[test]
    fn get_width_returns_correct_width() {
        // Arrange
        let sut = TetrominoPattern::new(create_test_pattern());

        // Act
        let result = sut.get_width();

        // Assert
        assert_eq!(result, 3);
    }

    #[rstest]
    #[case(I, TetrominoType::I)]
    #[case(O, TetrominoType::O)]
    #[case(T, TetrominoType::T)]
    #[case(Z, TetrominoType::Z)]
    #[case(S, TetrominoType::S)]
    #[case(J, TetrominoType::J)]
    #[case(L, TetrominoType::L)]
    fn number_to_tetromino_type_returns_correct_mapping(
        #[case] number: i8,
        #[case] expected_type: TetrominoType,
    ) {
        // Arrange
        let sut = TetrominoPattern::new(create_test_pattern());

        // Act
        let result = sut.number_to_tetromino_type(number);

        // Assert
        assert_eq!(result, expected_type);
    }

    fn create_test_pattern() -> Vec<Vec<i8>> {
        vec![vec![I, O, -1], vec![T, -1, Z]]
    }
}
