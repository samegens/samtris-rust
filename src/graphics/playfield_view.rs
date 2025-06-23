use crate::common::{Dimensions, Position};
use crate::tetromino::{TetrominoInstance, TetrominoType};

pub struct PlayfieldView<'a> {
    pub dimensions: Dimensions,
    pub grid: &'a Vec<Vec<Option<TetrominoType>>>,
    pub current_tetromino: Option<&'a TetrominoInstance>,
}

impl<'a> PlayfieldView<'a> {
    pub fn is_position_occupied(&self, position: Position) -> bool {
        if !self.dimensions.contains(position) {
            return false;
        }

        let x = position.x as u32;
        let y = position.y as u32;

        self.is_xy_occupied(x, y)
    }

    fn is_xy_occupied(&self, x: u32, y: u32) -> bool {
        self.grid[y as usize][x as usize].is_some()
    }

    pub fn get_tetromino_type_at(&self, position: Position) -> Option<TetrominoType> {
        if !self.dimensions.contains(position) {
            return None;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        self.grid[y][x]
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
        let mut grid = vec![vec![None; 3]; 3];
        grid[1][1] = Some(TetrominoType::O);
        let sut = PlayfieldView {
            dimensions,
            grid: &grid,
            current_tetromino: None,
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
        let grid = vec![vec![None; 3]; 3];
        let sut = PlayfieldView {
            dimensions,
            grid: &grid,
            current_tetromino: None,
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
        let mut grid = vec![vec![None; 3]; 3];
        grid[1][2] = Some(TetrominoType::T);
        let sut = PlayfieldView {
            dimensions,
            grid: &grid,
            current_tetromino: None,
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
        let grid = vec![vec![None; 3]; 3];
        let sut = PlayfieldView {
            dimensions,
            grid: &grid,
            current_tetromino: None,
        };

        // Act
        let result = sut.get_tetromino_type_at(Position::new(-1, 0));

        // Assert
        assert!(result.is_none());
    }
}
