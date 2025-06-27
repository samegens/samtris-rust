use crate::common::Position;
use crate::tetromino::TetrominoInstance;
use crate::tetromino::TetrominoType;

//TODO: Remove this when peek_next is used.
#[allow(dead_code)]
pub trait TetrominoGenerator {
    fn generate(&mut self, position: Position) -> TetrominoInstance;
    fn peek_next_type(&self) -> TetrominoType;
}
