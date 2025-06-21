use crate::common::Position;
use crate::tetromino::tetromino_instance::TetrominoInstance;

pub trait TetrominoGenerator {
    fn generate(&mut self, position: Position) -> TetrominoInstance;
}
