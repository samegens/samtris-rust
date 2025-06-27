use crate::common::Position;
use crate::tetromino::{
    TetrominoDefinitions, TetrominoGenerator, TetrominoInstance, TetrominoType,
};

pub struct FixedTetrominoGenerator {
    tetromino_type: TetrominoType,
    tetromino_definitions: TetrominoDefinitions,
}

impl FixedTetrominoGenerator {
    pub fn new(tetromino_type: TetrominoType) -> Self {
        Self {
            tetromino_type,
            tetromino_definitions: TetrominoDefinitions::new(),
        }
    }
}

impl TetrominoGenerator for FixedTetrominoGenerator {
    fn generate(&mut self, position: Position) -> TetrominoInstance {
        TetrominoInstance::new(self.tetromino_type, position, &self.tetromino_definitions)
    }

    fn peek_next(&self) -> TetrominoInstance {
        TetrominoInstance::new(
            self.tetromino_type,
            Position::new(0, 0),
            &self.tetromino_definitions,
        )
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn peek_next_returns_fixed_tetromino_type() {
        // Arrange
        let sut = FixedTetrominoGenerator::new(TetrominoType::T);

        // Act
        let result = sut.peek_next().get_type();

        // Assert
        assert_eq!(result, TetrominoType::T);
    }

    #[rstest]
    #[case(TetrominoType::O, Position::new(1, 1))]
    #[case(TetrominoType::S, Position::new(3, 4))]
    #[case(TetrominoType::L, Position::new(0, 7))]
    fn generate_returns_correct_tetromino_type_and_position(
        #[case] tetromino_type: TetrominoType,
        #[case] position: Position,
    ) {
        // Arrange
        let mut sut = FixedTetrominoGenerator::new(tetromino_type);

        // Act
        let result = sut.generate(position);

        // Assert
        assert_eq!(result.get_type(), tetromino_type);
        assert_eq!(result.get_position(), position);
    }

    #[test]
    fn peek_next_always_returns_same_type() {
        // Arrange
        let mut sut = FixedTetrominoGenerator::new(TetrominoType::Z);
        let position = Position::new(0, 0);
        let first_peek = sut.peek_next().get_type();
        sut.generate(position);

        // Act
        let second_peek = sut.peek_next().get_type();

        // Assert
        assert_eq!(first_peek, TetrominoType::Z);
        assert_eq!(second_peek, TetrominoType::Z);
    }
}
