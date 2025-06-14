use crate::position::Position;
use crate::rotation_index::RotationIndex;
use crate::tetromino_type::TetrominoType;

#[derive(Clone)]
pub struct TetrominoDefinition {
    tetromino_type: TetrominoType,
    rotations: Vec<Vec<Vec<u8>>>,
}

//TODO: remove allow dead_code when TetrominoDefinition is used by application code
#[allow(dead_code)]
impl TetrominoDefinition {
    pub fn new(tetromino_type: TetrominoType, rotations: Vec<Vec<Vec<u8>>>) -> Self {
        Self {
            tetromino_type,
            rotations,
        }
    }

    pub fn get_type(&self) -> TetrominoType {
        self.tetromino_type
    }

    pub fn get_nr_rotations(&self) -> usize {
        self.rotations.len()
    }

    pub fn create_o() -> Self {
        Self::new(
            TetrominoType::O,
            vec![vec![
                vec![0, 0, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0],
            ]],
        )
    }

    pub fn create_i() -> Self {
        Self::new(
            TetrominoType::I,
            vec![
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
            ],
        )
    }

    pub fn create_z() -> Self {
        Self::new(
            TetrominoType::Z,
            vec![
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
            ],
        )
    }

    pub fn create_s() -> Self {
        Self::new(
            TetrominoType::S,
            vec![
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
            ],
        )
    }

    pub fn create_t() -> Self {
        Self::new(
            TetrominoType::T,
            vec![
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
            ],
        )
    }

    pub fn create_j() -> Self {
        Self::new(
            TetrominoType::J,
            vec![
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
            ],
        )
    }

    pub fn create_l() -> Self {
        Self::new(
            TetrominoType::L,
            vec![
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
            ],
        )
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

    pub fn get_block_positions(&self, rotation: RotationIndex) -> Vec<Position> {
        let rotation_index: usize = rotation.into();
        if rotation_index >= self.rotations.len() {
            panic!(
                "Rotation index out of bounds: got {}, expected [0..{})",
                rotation_index,
                self.rotations.len()
            );
        }

        let matrix = &self.rotations[rotation_index];
        let mut positions = Vec::new();

        for (y, row) in matrix.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell != 0 {
                    positions.push(Position::new(x as i32, y as i32));
                }
            }
        }

        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![vec![1]]], RotationIndex::new(0, 1), Position::new(0, 0), true)]
    #[case(vec![vec![vec![1]]], RotationIndex::new(0, 1), Position::new(-1, -1), false)]
    #[case(vec![vec![vec![0]]], RotationIndex::new(0, 1), Position::new(0, 0), false)]
    #[case(vec![vec![vec![1]]], RotationIndex::new(0, 1), Position::new(1, 0), false)]
    #[case(vec![vec![vec![1]]], RotationIndex::new(0, 1), Position::new(0, 1), false)]
    #[case(vec![vec![vec![1]]], RotationIndex::new(1, 1), Position::new(0, 0), false)]
    #[case(vec![vec![vec![0]], vec![vec![1]]], RotationIndex::new(1, 2), Position::new(0, 0), true)]
    fn tetromino_has_block_at_position_matches_matrix_content(
        #[case] matrices: Vec<Vec<Vec<u8>>>,
        #[case] rotation: RotationIndex,
        #[case] position: Position,
        #[case] expected: bool,
    ) {
        // Arrange
        let tetromino = TetrominoDefinition::new(TetrominoType::O, matrices);

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
        assert_eq!(o_tetromino.get_nr_rotations(), 1);
        let last_rotation = RotationIndex::new(0, 1);
        assert!(!o_tetromino.has_block_at(Position::new(0, 0), last_rotation));
        assert!(o_tetromino.has_block_at(Position::new(1, 1), last_rotation));
        assert!(o_tetromino.has_block_at(Position::new(2, 2), last_rotation));
        assert!(!o_tetromino.has_block_at(Position::new(3, 3), last_rotation));
    }

    #[test]
    fn create_i_creates_proper_tetromino_definition() {
        // Act
        let i_tetromino = TetrominoDefinition::create_i();

        // Assert
        assert_eq!(i_tetromino.get_nr_rotations(), 2);
        let last_rotation = RotationIndex::new(1, 2);
        assert!(!i_tetromino.has_block_at(Position::new(0, 0), last_rotation));
        assert!(i_tetromino.has_block_at(Position::new(1, 1), last_rotation));
        assert!(!i_tetromino.has_block_at(Position::new(2, 2), last_rotation));
        assert!(!i_tetromino.has_block_at(Position::new(3, 3), last_rotation));
    }

    #[test]
    fn create_z_creates_proper_tetromino_definition() {
        // Act
        let z_tetromino = TetrominoDefinition::create_z();

        // Assert
        assert_eq!(z_tetromino.get_nr_rotations(), 2);
        let last_rotation = RotationIndex::new(1, 2);
        assert!(!z_tetromino.has_block_at(Position::new(0, 0), last_rotation));
        assert!(z_tetromino.has_block_at(Position::new(1, 1), last_rotation));
        assert!(!z_tetromino.has_block_at(Position::new(2, 2), last_rotation));
        assert!(!z_tetromino.has_block_at(Position::new(3, 3), last_rotation));
    }

    #[test]
    fn create_s_creates_proper_tetromino_definition() {
        // Act
        let s_tetromino = TetrominoDefinition::create_s();

        // Assert
        assert_eq!(s_tetromino.get_nr_rotations(), 2);
        let last_rotation = RotationIndex::new(1, 2);
        assert!(s_tetromino.has_block_at(Position::new(0, 0), last_rotation));
        assert!(s_tetromino.has_block_at(Position::new(1, 1), last_rotation));
        assert!(!s_tetromino.has_block_at(Position::new(2, 2), last_rotation));
        assert!(!s_tetromino.has_block_at(Position::new(3, 3), last_rotation));
    }

    #[test]
    fn create_t_creates_proper_tetromino_definition() {
        // Act
        let t_tetromino = TetrominoDefinition::create_t();

        // Assert
        assert_eq!(t_tetromino.get_nr_rotations(), 4);
        let last_rotation = RotationIndex::new(3, 4);
        assert!(!t_tetromino.has_block_at(Position::new(0, 0), last_rotation));
        assert!(t_tetromino.has_block_at(Position::new(1, 1), last_rotation));
        assert!(!t_tetromino.has_block_at(Position::new(2, 2), last_rotation));
        assert!(!t_tetromino.has_block_at(Position::new(3, 3), last_rotation));
    }

    #[test]
    fn create_j_creates_proper_tetromino_definition() {
        // Act
        let j_tetromino = TetrominoDefinition::create_j();

        // Assert
        assert_eq!(j_tetromino.get_nr_rotations(), 4);
        let last_rotation = RotationIndex::new(3, 4);
        assert!(!j_tetromino.has_block_at(Position::new(0, 0), last_rotation));
        assert!(!j_tetromino.has_block_at(Position::new(1, 1), last_rotation));
        assert!(j_tetromino.has_block_at(Position::new(2, 2), last_rotation));
        assert!(!j_tetromino.has_block_at(Position::new(3, 3), last_rotation));
    }

    #[test]
    fn create_l_creates_proper_tetromino_definition() {
        // Act
        let l_tetromino = TetrominoDefinition::create_l();

        // Assert
        assert_eq!(l_tetromino.get_nr_rotations(), 4);
        let last_rotation = RotationIndex::new(3, 4);
        assert!(!l_tetromino.has_block_at(Position::new(0, 0), last_rotation));
        assert!(!l_tetromino.has_block_at(Position::new(1, 1), last_rotation));
        assert!(l_tetromino.has_block_at(Position::new(2, 2), last_rotation));
        assert!(!l_tetromino.has_block_at(Position::new(3, 3), last_rotation));
    }

    #[test]
    #[should_panic(expected = "Rotation index out of bounds")]
    fn get_blocks_panics_with_invalid_rotation_index() {
        // Arrange
        let definition = TetrominoDefinition::create_o(); // O-piece has 1 rotation (index 0 only)
        let invalid_rotation = RotationIndex::new(1, 2); // Index 1, but O-piece only has 1 rotation

        // Act & Assert (panic expected)
        definition.get_block_positions(invalid_rotation);
    }
}
