use crate::common::{Dimensions, Position};
use crate::game_logic::PlayfieldGrid;
use crate::tetromino::{TetrominoInstance, TetrominoType};

pub struct PlayfieldView<'a> {
    pub dimensions: Dimensions,
    pub grid: &'a PlayfieldGrid,
    pub current_tetromino: Option<&'a TetrominoInstance>,
    pub full_lines: Vec<u32>,
    pub show_blinking_lines: bool,
}

impl<'a> PlayfieldView<'a> {
    pub fn is_position_occupied(&self, position: Position) -> bool {
        self.grid.is_position_occupied(position)
    }

    pub fn get_tetromino_type_at(&self, position: Position) -> Option<TetrominoType> {
        if !self.dimensions.contains(position) {
            return None;
        }

        self.grid.get(position).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Dimensions;

    #[test]
    fn is_position_occupied_returns_true_for_occupied_position() {
        // Arrange
        let dimensions = Dimensions::new(3, 3);
        let mut grid = PlayfieldGrid::new(dimensions);
        grid.set(Position::new(1, 1), Some(TetrominoType::O));
        let sut = PlayfieldView {
            dimensions,
            grid: &grid,
            current_tetromino: None,
            full_lines: vec![],
            show_blinking_lines: false,
        };

        // Act
        let result = sut.is_position_occupied(Position::new(1, 1));

        // Assert
        assert!(result);
    }

    #[test]
    fn is_position_occupied_returns_false_for_out_of_bounds() {
        // Arrange
        let dimensions = Dimensions::new(3, 3);
        let grid = PlayfieldGrid::new(dimensions);
        let sut = PlayfieldView {
            dimensions,
            grid: &grid,
            current_tetromino: None,
            full_lines: vec![],
            show_blinking_lines: false,
        };

        // Act
        let result = sut.is_position_occupied(Position::new(-1, 0));

        // Assert
        assert!(!result);
    }

    #[test]
    fn get_tetromino_type_at_returns_correct_type() {
        // Arrange
        let dimensions = Dimensions::new(3, 3);
        let mut grid = PlayfieldGrid::new(dimensions);
        grid.set(Position::new(2, 1), Some(TetrominoType::T));
        let sut = PlayfieldView {
            dimensions,
            grid: &grid,
            current_tetromino: None,
            full_lines: vec![],
            show_blinking_lines: false,
        };

        // Act
        let result = sut.get_tetromino_type_at(Position::new(2, 1));

        // Assert
        assert_eq!(result, Some(TetrominoType::T));
    }

    #[test]
    fn get_tetromino_type_at_returns_none_for_out_of_bounds() {
        // Arrange
        let dimensions = Dimensions::new(3, 3);
        let grid = PlayfieldGrid::new(dimensions);
        let sut = PlayfieldView {
            dimensions,
            grid: &grid,
            current_tetromino: None,
            full_lines: vec![],
            show_blinking_lines: false,
        };

        // Act
        let result = sut.get_tetromino_type_at(Position::new(-1, 0));

        // Assert
        assert!(result.is_none());
    }
}
