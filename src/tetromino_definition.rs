use crate::position::Position;
use crate::rotation_index::RotationIndex;

pub struct TetrominoDefinition {
    rotations: Vec<Vec<Vec<u8>>>,
}

impl TetrominoDefinition {
    pub fn new(rotations: Vec<Vec<Vec<u8>>>) -> Self {
        Self { rotations }
    }

    pub fn get_next_rotation_index_clockwise(
        &self,
        current_rotation_index: RotationIndex,
    ) -> RotationIndex {
        let next_index = (current_rotation_index.value() + 1) % self.rotations.len();
        RotationIndex::new(next_index)
    }

    pub fn get_next_rotation_index_counterclockwise(
        &self,
        current: RotationIndex,
    ) -> RotationIndex {
        let next_index = (current.value() + self.rotations.len() - 1) % self.rotations.len();
        RotationIndex::new(next_index)
    }

    pub fn has_block_at(&self, position: Position, rotation: RotationIndex) -> bool {
        if rotation.value() >= self.rotations.len() {
            return false;
        }

        let matrix = &self.rotations[rotation.value()];

        // Check if position is within matrix bounds
        if position.y < 0 || position.x < 0 {
            return false;
        }

        let y = position.y as usize;
        let x = position.x as usize;

        if y >= matrix.len() || x >= matrix[y].len() {
            return false;
        }

        matrix[y][x] != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn tetromino_with_one_rotation_clockwise_returns_same_rotation_index() {
        // Arrange
        let minimal_matrix = vec![vec![0]];
        let single_rotation_tetromino = TetrominoDefinition::new(vec![minimal_matrix]);
        let initial_rotation = RotationIndex::new(0);

        // Act
        let next_rotation =
            single_rotation_tetromino.get_next_rotation_index_clockwise(initial_rotation);

        // Assert
        assert_eq!(next_rotation, initial_rotation);
    }

    #[test]
    fn tetromino_with_two_rotations_clockwise_cycles_to_next_rotation() {
        // Arrange
        let minimal_matrix = vec![vec![0]];
        let two_rotation_tetromino =
            TetrominoDefinition::new(vec![minimal_matrix.clone(), minimal_matrix]);
        let rotation_zero = RotationIndex::new(0);

        // Act
        let next_rotation = two_rotation_tetromino.get_next_rotation_index_clockwise(rotation_zero);

        // Assert
        assert_eq!(next_rotation, RotationIndex::new(1));
    }

    #[test]
    fn tetromino_with_one_rotation_counterclockwise_returns_same_rotation_index() {
        // Arrange
        let minimal_matrix = vec![vec![0]];
        let single_rotation_tetromino = TetrominoDefinition::new(vec![minimal_matrix]);
        let initial_rotation = RotationIndex::new(0);

        // Act
        let next_rotation =
            single_rotation_tetromino.get_next_rotation_index_counterclockwise(initial_rotation);

        // Assert
        assert_eq!(next_rotation, initial_rotation);
    }

    #[test]
    fn tetromino_with_two_rotations_counterclockwise_cycles_to_next_rotation() {
        // Arrange
        let minimal_matrix = vec![vec![0]];
        let two_rotation_tetromino =
            TetrominoDefinition::new(vec![minimal_matrix.clone(), minimal_matrix]);
        let rotation_zero = RotationIndex::new(0);

        // Act
        let next_rotation =
            two_rotation_tetromino.get_next_rotation_index_counterclockwise(rotation_zero);

        // Assert
        assert_eq!(next_rotation, RotationIndex::new(1));
    }

    #[rstest]
    #[case(vec![vec![vec![1]]], RotationIndex::new(0), Position::new(0, 0), true)]
    #[case(vec![vec![vec![0]]], RotationIndex::new(0), Position::new(0, 0), false)]
    #[case(vec![vec![vec![1]]], RotationIndex::new(0), Position::new(1, 0), false)]
    #[case(vec![vec![vec![1]]], RotationIndex::new(0), Position::new(0, 1), false)]
    #[case(vec![vec![vec![1]]], RotationIndex::new(1), Position::new(0, 0), false)]
    #[case(vec![vec![vec![0]], vec![vec![1]]], RotationIndex::new(1), Position::new(0, 0), true)]
    fn tetromino_has_block_at_position_matches_matrix_content(
        #[case] matrices: Vec<Vec<Vec<u8>>>,
        #[case] rotation: RotationIndex,
        #[case] position: Position,
        #[case] expected: bool,
    ) {
        // Arrange
        let tetromino = TetrominoDefinition::new(matrices);

        // Act
        let has_block = tetromino.has_block_at(position, rotation);

        // Assert
        assert_eq!(has_block, expected);
    }
}
