use crate::common::Position;
use crate::common::RotationIndex;
use crate::tetromino::tetromino_definition::TetrominoDefinition;
use crate::tetromino::tetromino_definitions::TetrominoDefinitions;
use crate::tetromino::tetromino_type::TetrominoType;

#[derive(Clone)]
pub struct TetrominoInstance {
    tetromino_type: TetrominoType,
    definition: TetrominoDefinition,
    position: Position,
    rotation_index: RotationIndex,
}

impl TetrominoInstance {
    pub fn new(
        tetromino_type: TetrominoType,
        position: Position,
        tetromino_definitions: &TetrominoDefinitions,
    ) -> Self {
        let definition = tetromino_definitions.get(tetromino_type).clone();
        let nr_rotations = definition.get_nr_rotations();
        Self {
            tetromino_type,
            definition,
            position,
            rotation_index: RotationIndex::new(0, nr_rotations),
        }
    }

    pub fn get_type(&self) -> TetrominoType {
        self.tetromino_type
    }

    #[cfg(test)]
    pub fn get_position(&self) -> Position {
        self.position
    }

    #[cfg(test)]
    pub fn get_rotation_index(&self) -> RotationIndex {
        self.rotation_index
    }

    pub fn get_world_blocks(&self) -> Vec<Position> {
        self.definition
            .get_block_positions(self.rotation_index) // Get local blocks from definition
            .into_iter()
            .map(|block_position| self.position + block_position) // Transform to world coordinates
            .collect()
    }

    pub fn move_down(&mut self) {
        self.position = self.position.translate(0, 1);
    }

    pub fn move_left(&mut self) {
        self.position = self.position.translate(-1, 0);
    }

    pub fn move_right(&mut self) {
        self.position = self.position.translate(1, 0);
    }

    pub fn rotate_clockwise(&mut self) {
        self.rotation_index.rotate_clockwise();
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.rotation_index.rotate_counterclockwise();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetromino::tetromino_definitions::TetrominoDefinitions;

    #[test]
    fn tetromino_instance_starts_with_given_position_and_rotation_zero() {
        // Arrange
        let position = Position::new(5, 10);
        let definitions = TetrominoDefinitions::new();

        // Act
        let sut = TetrominoInstance::new(TetrominoType::O, position, &definitions);

        // Assert
        assert_eq!(sut.get_type(), TetrominoType::O);
        assert_eq!(sut.get_position(), position);
        assert_eq!(sut.get_rotation_index(), RotationIndex::new(0, 1)); // O-piece has 1 rotation
    }

    #[test]
    fn tetromino_instance_returns_correct_world_blocks() {
        // Arrange
        let position = Position::new(5, 5);
        let definitions = TetrominoDefinitions::new();
        let sut = TetrominoInstance::new(TetrominoType::O, position, &definitions);

        // Act
        let result = sut.get_world_blocks();

        // Assert
        let expected_blocks = vec![
            Position::new(6, 6), // (1,1) + (5,5)
            Position::new(7, 6), // (2,1) + (5,5)
            Position::new(6, 7), // (1,2) + (5,5)
            Position::new(7, 7), // (2,2) + (5,5)
        ];
        assert_eq!(result, expected_blocks);
    }

    #[test]
    fn move_down_increases_y_coordinate() {
        // Arrange
        let position = Position::new(5, 5);
        let definitions = TetrominoDefinitions::new();
        let mut sut = TetrominoInstance::new(TetrominoType::T, position, &definitions);

        // Act
        sut.move_down();

        // Assert
        assert_eq!(sut.get_position(), Position::new(5, 6));
        assert_eq!(sut.get_rotation_index(), RotationIndex::new(0, 4));
    }

    #[test]
    fn move_left_decreases_x_coordinate() {
        // Arrange
        let position = Position::new(5, 5);
        let definitions = TetrominoDefinitions::new();
        let mut sut = TetrominoInstance::new(TetrominoType::T, position, &definitions);

        // Act
        sut.move_left();

        // Assert
        assert_eq!(sut.get_position(), Position::new(4, 5));
        assert_eq!(sut.get_rotation_index(), RotationIndex::new(0, 4));
    }

    #[test]
    fn move_right_increases_x_coordinate() {
        // Arrange
        let position = Position::new(5, 5);
        let definitions = TetrominoDefinitions::new();
        let mut sut = TetrominoInstance::new(TetrominoType::T, position, &definitions);

        // Act
        sut.move_right();

        // Assert
        assert_eq!(sut.get_position(), Position::new(6, 5));
        assert_eq!(sut.get_rotation_index(), RotationIndex::new(0, 4));
    }

    #[test]
    fn rotate_clockwise_advances_rotation_index() {
        // Arrange
        let position = Position::new(5, 5);
        let definitions = TetrominoDefinitions::new();
        let mut sut = TetrominoInstance::new(TetrominoType::T, position, &definitions); // T-piece has 4 rotations

        // Act
        sut.rotate_clockwise();

        // Assert
        assert_eq!(sut.get_rotation_index(), RotationIndex::new(1, 4));
        assert_eq!(sut.get_position(), Position::new(5, 5));
    }

    #[test]
    fn rotate_counterclockwise_decreases_rotation_index() {
        // Arrange
        let position = Position::new(5, 5);
        let definitions = TetrominoDefinitions::new();
        let mut sut = TetrominoInstance::new(TetrominoType::T, position, &definitions);

        // Act
        sut.rotate_counterclockwise();

        // Assert
        assert_eq!(sut.get_rotation_index(), RotationIndex::new(3, 4));
        assert_eq!(sut.get_position(), Position::new(5, 5));
    }
}
