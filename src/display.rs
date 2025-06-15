use crate::position::Position;
use crate::tetromino_type::TetrominoType;

/// Abstract display interface
pub trait Display {
    type Error;

    fn clear(&mut self) -> Result<(), Self::Error>;
    fn draw_block(
        &mut self,
        position: Position,
        tetromino_type: TetrominoType,
    ) -> Result<(), Self::Error>;
    fn draw_playfield_border(&mut self) -> Result<(), Self::Error>;
    fn present(&mut self) -> Result<(), Self::Error>;
}
