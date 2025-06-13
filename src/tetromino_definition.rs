use crate::position::Position;
use crate::rotation_index::RotationIndex;

//TODO: remove allow dead_code when TetrominoDefinition is used by application code
#[allow(dead_code)]
pub struct TetrominoDefinition {
    rotations: Vec<Vec<Vec<u8>>>,
}

//TODO: remove allow dead_code when TetrominoDefinition is used by application code
#[allow(dead_code)]
impl TetrominoDefinition {
    pub fn new(rotations: Vec<Vec<Vec<u8>>>) -> Self {
        Self { rotations }
    }

    pub fn create_o() -> Self {
        Self::new(vec![vec![
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ]])
    }

    pub fn create_i() -> Self {
        Self::new(vec![
            vec![
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
        ])
    }

    pub fn create_z() -> Self {
        Self::new(vec![
            vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0],
            ],
            vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 0],
                vec![0, 0, 0, 0],
            ],
        ])
    }

    pub fn create_s() -> Self {
        Self::new(vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 0],
                vec![1, 1, 0, 0],
                vec![0, 0, 0, 0],
            ],
            vec![
                vec![1, 0, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 0, 0],
            ],
        ])
    }

    pub fn create_t() -> Self {
        Self::new(vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 1, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 1, 0, 0],
            ],
        ])
    }

    pub fn create_j() -> Self {
        Self::new(vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 1, 0],
            ],
        ])
    }

    pub fn create_l() -> Self {
        Self::new(vec![
            vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 1, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 1, 1, 0],
                vec![1, 0, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 1, 0, 0],
            ],
            vec![
                vec![0, 0, 0, 0],
                vec![0, 0, 1, 0],
                vec![1, 1, 1, 0],
                vec![0, 0, 0, 0],
            ],
        ])
    }

    pub fn get_next_rotation_index_clockwise(
        &self,
        rotation_index: RotationIndex,
    ) -> RotationIndex {
        let next_index = (usize::from(rotation_index) + 1) % self.rotations.len();
        RotationIndex::new(next_index)
    }

    pub fn get_next_rotation_index_counterclockwise(
        &self,
        rotation_index: RotationIndex,
    ) -> RotationIndex {
        let next_index =
            (usize::from(rotation_index) + self.rotations.len() - 1) % self.rotations.len();
        RotationIndex::new(next_index)
    }

    pub fn has_block_at(&self, position: Position, rotation_index: RotationIndex) -> bool {
        if usize::from(rotation_index) >= self.rotations.len() {
            return false;
        }

        let matrix = &self.rotations[usize::from(rotation_index)];

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
    #[case(vec![vec![vec![1]]], RotationIndex::new(0), Position::new(-1, -1), false)]
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

    #[test]
    fn create_o_creates_proper_tetromino_definition() {
        // Act
        let o_tetromino = TetrominoDefinition::create_o();

        // Assert
        assert_eq!(
            o_tetromino.get_next_rotation_index_clockwise(RotationIndex::new(0)),
            RotationIndex::new(0)
        );
        assert!(!o_tetromino.has_block_at(Position::new(0, 0), RotationIndex::new(0)));
        assert!(o_tetromino.has_block_at(Position::new(1, 1), RotationIndex::new(0)));
        assert!(o_tetromino.has_block_at(Position::new(2, 2), RotationIndex::new(0)));
        assert!(!o_tetromino.has_block_at(Position::new(3, 3), RotationIndex::new(0)));
    }

    #[test]
    fn create_i_creates_proper_tetromino_definition() {
        // Act
        let i_tetromino = TetrominoDefinition::create_i();

        // Assert
        assert_eq!(
            i_tetromino.get_next_rotation_index_clockwise(RotationIndex::new(1)),
            RotationIndex::new(0)
        );
        assert!(!i_tetromino.has_block_at(Position::new(0, 0), RotationIndex::new(1)));
        assert!(i_tetromino.has_block_at(Position::new(1, 1), RotationIndex::new(1)));
        assert!(!i_tetromino.has_block_at(Position::new(2, 2), RotationIndex::new(1)));
        assert!(!i_tetromino.has_block_at(Position::new(3, 3), RotationIndex::new(1)));
    }

    #[test]
    fn create_z_creates_proper_tetromino_definition() {
        // Act
        let z_tetromino = TetrominoDefinition::create_z();

        // Assert
        assert_eq!(
            z_tetromino.get_next_rotation_index_clockwise(RotationIndex::new(1)),
            RotationIndex::new(0)
        );
        assert!(!z_tetromino.has_block_at(Position::new(0, 0), RotationIndex::new(1)));
        assert!(z_tetromino.has_block_at(Position::new(1, 1), RotationIndex::new(1)));
        assert!(!z_tetromino.has_block_at(Position::new(2, 2), RotationIndex::new(1)));
        assert!(!z_tetromino.has_block_at(Position::new(3, 3), RotationIndex::new(1)));
    }

    #[test]
    fn create_s_creates_proper_tetromino_definition() {
        // Act
        let s_tetromino = TetrominoDefinition::create_s();

        // Assert
        assert_eq!(
            s_tetromino.get_next_rotation_index_clockwise(RotationIndex::new(1)),
            RotationIndex::new(0)
        );
        assert!(s_tetromino.has_block_at(Position::new(0, 0), RotationIndex::new(1)));
        assert!(s_tetromino.has_block_at(Position::new(1, 1), RotationIndex::new(1)));
        assert!(!s_tetromino.has_block_at(Position::new(2, 2), RotationIndex::new(1)));
        assert!(!s_tetromino.has_block_at(Position::new(3, 3), RotationIndex::new(1)));
    }

    #[test]
    fn create_t_creates_proper_tetromino_definition() {
        // Act
        let t_tetromino = TetrominoDefinition::create_t();

        // Assert
        assert_eq!(
            t_tetromino.get_next_rotation_index_clockwise(RotationIndex::new(3)),
            RotationIndex::new(0)
        );
        assert!(!t_tetromino.has_block_at(Position::new(0, 0), RotationIndex::new(3)));
        assert!(t_tetromino.has_block_at(Position::new(1, 1), RotationIndex::new(3)));
        assert!(!t_tetromino.has_block_at(Position::new(2, 2), RotationIndex::new(3)));
        assert!(!t_tetromino.has_block_at(Position::new(3, 3), RotationIndex::new(3)));
    }

    #[test]
    fn create_j_creates_proper_tetromino_definition() {
        // Act
        let j_tetromino = TetrominoDefinition::create_j();

        // Assert
        assert_eq!(
            j_tetromino.get_next_rotation_index_clockwise(RotationIndex::new(3)),
            RotationIndex::new(0)
        );
        assert!(!j_tetromino.has_block_at(Position::new(0, 0), RotationIndex::new(3)));
        assert!(!j_tetromino.has_block_at(Position::new(1, 1), RotationIndex::new(3)));
        assert!(j_tetromino.has_block_at(Position::new(2, 2), RotationIndex::new(3)));
        assert!(!j_tetromino.has_block_at(Position::new(3, 3), RotationIndex::new(3)));
    }

    #[test]
    fn create_l_creates_proper_tetromino_definition() {
        // Act
        let l_tetromino = TetrominoDefinition::create_l();

        // Assert
        assert_eq!(
            l_tetromino.get_next_rotation_index_clockwise(RotationIndex::new(3)),
            RotationIndex::new(0)
        );
        assert!(!l_tetromino.has_block_at(Position::new(0, 0), RotationIndex::new(3)));
        assert!(!l_tetromino.has_block_at(Position::new(1, 1), RotationIndex::new(3)));
        assert!(l_tetromino.has_block_at(Position::new(2, 2), RotationIndex::new(3)));
        assert!(!l_tetromino.has_block_at(Position::new(3, 3), RotationIndex::new(3)));
    }
}
