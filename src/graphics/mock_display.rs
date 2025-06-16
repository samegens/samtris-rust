use crate::common::Position;
use crate::graphics::Color;
use crate::graphics::Display;
use crate::tetromino_type::TetrominoType;

#[derive(Debug, Default)]
pub struct MockDisplay {
    pub cleared: bool,
    pub drawn_blocks: Vec<(Position, TetrominoType)>,
    pub drawn_rectangles: Vec<(u32, u32, u32, u32, Color)>,
    pub presented: bool,
}

impl MockDisplay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.cleared = false;
        self.drawn_blocks.clear();
        self.drawn_rectangles.clear();
        self.presented = false;
    }
}

impl Display for MockDisplay {
    type Error = ();

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.cleared = true;
        self.drawn_blocks.clear();
        self.drawn_rectangles.clear();
        Ok(())
    }

    fn draw_block(
        &mut self,
        position: Position,
        tetromino_type: TetrominoType,
    ) -> Result<(), Self::Error> {
        self.drawn_blocks.push((position, tetromino_type));
        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
    ) -> Result<(), Self::Error> {
        self.drawn_rectangles.push((x, y, width, height, color));
        Ok(())
    }

    fn present(&mut self) -> Result<(), Self::Error> {
        self.presented = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_display_starts_empty() {
        // Act
        let display = MockDisplay::new();

        // Assert
        assert!(!display.cleared);
        assert!(display.drawn_blocks.is_empty());
        assert!(display.drawn_rectangles.is_empty());
        assert!(!display.presented);
    }

    #[test]
    fn clear_sets_cleared_flag_and_empties_collections() {
        // Arrange
        let mut display = MockDisplay::new();
        display
            .drawn_blocks
            .push((Position::new(1, 1), TetrominoType::O));
        display.drawn_rectangles.push((0, 0, 10, 10, Color::RED));

        // Act
        let result = display.clear();

        // Assert
        assert!(result.is_ok());
        assert!(display.cleared);
        assert!(display.drawn_blocks.is_empty());
        assert!(display.drawn_rectangles.is_empty());
    }

    #[test]
    fn draw_block_records_position_and_type() {
        // Arrange
        let mut display = MockDisplay::new();
        let position = Position::new(5, 10);
        let tetromino_type = TetrominoType::T;

        // Act
        let result = display.draw_block(position, tetromino_type);

        // Assert
        assert!(result.is_ok());
        assert_eq!(display.drawn_blocks.len(), 1);
        assert_eq!(display.drawn_blocks[0], (position, tetromino_type));
    }

    #[test]
    fn draw_rectangle_records_parameters() {
        // Arrange
        let mut display = MockDisplay::new();

        // Act
        let result = display.draw_rectangle(10, 20, 100, 200, Color::BLUE);

        // Assert
        assert!(result.is_ok());
        assert_eq!(display.drawn_rectangles.len(), 1);
        assert_eq!(display.drawn_rectangles[0], (10, 20, 100, 200, Color::BLUE));
    }

    #[test]
    fn present_sets_presented_flag() {
        // Arrange
        let mut display = MockDisplay::new();

        // Act
        let result = display.present();

        // Assert
        assert!(result.is_ok());
        assert!(display.presented);
    }

    #[test]
    fn reset_clears_all_state() {
        // Arrange
        let mut display = MockDisplay::new();
        display.cleared = true;
        display
            .drawn_blocks
            .push((Position::new(1, 1), TetrominoType::O));
        display.drawn_rectangles.push((0, 0, 10, 10, Color::RED));
        display.presented = true;

        // Act
        display.reset();

        // Assert
        assert!(!display.cleared);
        assert!(display.drawn_blocks.is_empty());
        assert!(display.drawn_rectangles.is_empty());
        assert!(!display.presented);
    }
}
