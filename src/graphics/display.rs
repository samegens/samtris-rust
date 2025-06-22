use crate::common::Position;
use crate::graphics::Color;
use crate::tetromino::TetrominoType;

/// Abstract display interface
pub trait Display {
    fn clear(&mut self) -> Result<(), String>;

    fn draw_block(
        &mut self,
        position: Position,
        tetromino_type: TetrominoType,
    ) -> Result<(), String>;

    fn draw_rectangle(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
    ) -> Result<(), String>;

    fn present(&mut self) -> Result<(), String>;
}
