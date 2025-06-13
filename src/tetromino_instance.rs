use crate::position::Position;
use crate::rotation_index::RotationIndex;
use crate::tetromino_definition::TetrominoDefinition;
use crate::tetromino_definitions::TetrominoDefinitions;
use crate::tetromino_type::TetrominoType;

pub struct TetrominoInstance<'a> {
    tetromino_type: TetrominoType,
    definition: &'a TetrominoDefinition,
    position: Position,
    rotation_index: RotationIndex,
}

//TODO: remove allow dead_code when TetrominoDefinition is used by application code
#[allow(dead_code)]
impl<'a> TetrominoInstance<'a> {
    pub fn new(
        tetromino_type: TetrominoType,
        position: Position,
        tetromino_definitions: &'a TetrominoDefinitions,
    ) -> Self {
        let definition = tetromino_definitions.get(tetromino_type);
        Self {
            tetromino_type,
            definition,
            position,
            rotation_index: RotationIndex::new(0, definition.get_nr_rotations()),
        }
    }

    pub fn get_type(&self) -> TetrominoType {
        self.tetromino_type
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetromino_definitions::TetrominoDefinitions;

    #[test]
    fn tetromino_instance_starts_with_given_position_and_rotation_zero() {
        // Arrange
        let position = Position::new(5, 10);
        let definitions = TetrominoDefinitions::new();

        // Act
        let instance = TetrominoInstance::new(TetrominoType::O, position, &definitions);

        // Assert
        assert_eq!(instance.get_type(), TetrominoType::O);
        assert_eq!(instance.get_position(), position);
        assert_eq!(instance.get_rotation_index(), RotationIndex::new(0, 1)); // O-piece has 1 rotation
    }

    #[test]
    fn tetromino_instance_returns_correct_world_blocks() {
        // Arrange
        let position = Position::new(5, 5);
        let definitions = TetrominoDefinitions::new();
        let instance = TetrominoInstance::new(TetrominoType::O, position, &definitions);

        // Act
        let world_blocks = instance.get_world_blocks();

        // Assert
        let expected_blocks = vec![
            Position::new(6, 6), // (1,1) + (5,5)
            Position::new(7, 6), // (2,1) + (5,5)
            Position::new(6, 7), // (1,2) + (5,5)
            Position::new(7, 7), // (2,2) + (5,5)
        ];
        assert_eq!(world_blocks, expected_blocks);
    }
}
