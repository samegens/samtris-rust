use crate::common::Position;
use crate::graphics::Color;
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

    fn draw_rectangle(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: Color,
    ) -> Result<(), Self::Error>;

    fn present(&mut self) -> Result<(), Self::Error>;
}
