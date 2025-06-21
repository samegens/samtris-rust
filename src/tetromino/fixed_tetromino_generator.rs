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
}
