use crate::dimensions::Dimensions;
use crate::position::Position;
use crate::tetromino_definitions::TetrominoDefinitions;
use crate::tetromino_instance::TetrominoInstance;
use crate::tetromino_type::TetrominoType;

pub struct Playfield {
    dimensions: Dimensions,
    grid: Vec<Vec<Option<TetrominoType>>>,
}

impl Playfield {
    pub fn new(dimensions: Dimensions) -> Self {
        let grid = vec![vec![None; dimensions.width]; dimensions.height];
        Self { dimensions, grid }
    }

    pub fn get_dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn is_position_occupied(&self, position: Position) -> bool {
        if !self.dimensions.contains(position) {
            return false;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        self.grid[y][x].is_some()
    }

    pub fn place_tetromino(&mut self, tetromino: &TetrominoInstance) {
        let tetromino_type = tetromino.get_type();
        let world_blocks = tetromino.get_world_blocks();

        for position in world_blocks {
            if self.dimensions.contains(position) {
                let x = position.x as usize;
                let y = position.y as usize;

                self.grid[y][x] = Some(tetromino_type);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn new_playfield_has_correct_dimensions() {
        // Arrange
        let dimensions = Dimensions::new(7, 11);

        // Act
        let sut = Playfield::new(dimensions);

        // Assert
        assert_eq!(sut.get_dimensions(), dimensions);
    }

    #[rstest]
    #[case(Position::new(-1, 0))]
    #[case(Position::new(10, 0))]
    #[case(Position::new(10, 20))]
    #[case(Position::new(0, 20))]
    fn is_position_occupied_handles_out_of_bounds(#[case] position: Position) {
        // Arrange
        let dimensions = Dimensions::new(10, 20);
        let sut = Playfield::new(dimensions);

        // Act
        let result = sut.is_position_occupied(position);

        // Assert
        assert!(!result)
    }

    #[test]
    fn place_tetromino_marks_all_block_positions_occupied() {
        // Arrange
        let dimensions = Dimensions::new(10, 20);
        let mut sut = Playfield::new(dimensions);
        let definitions = TetrominoDefinitions::new();
        let tetromino = TetrominoInstance::new(TetrominoType::O, Position::new(5, 5), &definitions);

        // Act
        sut.place_tetromino(&tetromino);

        // Assert
        assert!(!sut.is_position_occupied(Position::new(5, 5)));
        assert!(sut.is_position_occupied(Position::new(5 + 1, 5 + 1)));
        assert!(sut.is_position_occupied(Position::new(5 + 2, 5 + 1)));
        assert!(sut.is_position_occupied(Position::new(5 + 1, 5 + 2)));
        assert!(sut.is_position_occupied(Position::new(5 + 2, 5 + 2)));
        assert!(!sut.is_position_occupied(Position::new(5 + 3, 5 + 3)));
    }
}
