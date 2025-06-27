use crate::common::Position;
use crate::tetromino::{
    TetrominoDefinitions, TetrominoGenerator, TetrominoInstance, TetrominoType,
};
use rand::Rng;
use strum::IntoEnumIterator;

pub struct RandomTetrominoGenerator {
    rng: rand::rngs::ThreadRng,
    tetromino_definitions: TetrominoDefinitions,
    next_tetromino_type: TetrominoType,
}

impl RandomTetrominoGenerator {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let next_tetromino_type = Self::select_random_type_from_rnd(&mut rng);
        Self {
            rng,
            tetromino_definitions: TetrominoDefinitions::new(),
            next_tetromino_type,
        }
    }

    fn select_random_type(&mut self) -> TetrominoType {
        Self::select_random_type_from_rnd(&mut self.rng)
    }

    fn select_random_type_from_rnd(rand: &mut rand::rngs::ThreadRng) -> TetrominoType {
        let types: Vec<TetrominoType> = TetrominoType::iter().collect();
        types[rand.random_range(0..types.len())]
    }
}

impl TetrominoGenerator for RandomTetrominoGenerator {
    fn generate(&mut self, position: Position) -> TetrominoInstance {
        let tetromino_type = self.next_tetromino_type;
        self.next_tetromino_type = self.select_random_type();
        TetrominoInstance::new(tetromino_type, position, &self.tetromino_definitions)
    }

    fn peek_next(&self) -> TetrominoInstance {
        TetrominoInstance::new(
            self.next_tetromino_type,
            Position::new(0, 0),
            &self.tetromino_definitions,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tetromino::random_tetromino_generator::RandomTetrominoGenerator;
    use std::collections::HashSet;
    use strum::IntoEnumIterator;

    #[test]
    fn generate_creates_tetromino_instance_at_given_position() {
        // Arrange
        let mut sut = RandomTetrominoGenerator::new();
        let position = Position::new(5, 10);

        // Act
        let result = sut.generate(position);

        // Assert
        assert_eq!(result.get_position(), position);
        // Verify it's one of the valid tetromino types
        let valid_types: Vec<TetrominoType> = TetrominoType::iter().collect();
        assert!(valid_types.contains(&result.get_type()));
    }

    #[test]
    fn generate_produces_different_types_over_multiple_calls() {
        // Arrange
        let mut sut = RandomTetrominoGenerator::new();
        let position = Position::new(0, 0);
        let mut generated_types = HashSet::new();

        // Act
        for _ in 0..100 {
            let tetromino = sut.generate(position);
            generated_types.insert(tetromino.get_type());
        }

        // Assert
        assert!(generated_types.len() > 1);
    }

    #[test]
    fn next_returns_peeked_tetromino_type() {
        // Arrange
        let mut sut = RandomTetrominoGenerator::new();
        let position = Position::new(5, 10);

        // Act
        let peeked_type = sut.peek_next().get_type();
        let generated = sut.generate(position);

        // Assert
        assert_eq!(generated.get_type(), peeked_type);
        assert_eq!(generated.get_position(), position);
    }
}
