// src/sdl_display.rs
use crate::common::Position;
use crate::graphics::Color;
use crate::graphics::Display;
use crate::tetromino_type::TetrominoType;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct SdlDisplay {
    canvas: Canvas<Window>,
    block_size_in_pixels: u32,
}

impl SdlDisplay {
    pub fn new(canvas: Canvas<Window>, block_size_in_pixels: u32) -> Self {
        Self {
            canvas,
            block_size_in_pixels,
        }
    }

    fn tetromino_color(&self, tetromino_type: TetrominoType) -> SdlColor {
        match tetromino_type {
            TetrominoType::I => SdlColor::RGB(0, 255, 255),
            TetrominoType::O => SdlColor::RGB(255, 255, 0),
            TetrominoType::T => SdlColor::RGB(128, 0, 128),
            TetrominoType::Z => SdlColor::RGB(255, 0, 0),
            TetrominoType::S => SdlColor::RGB(0, 255, 0),
            TetrominoType::J => SdlColor::RGB(0, 0, 255),
            TetrominoType::L => SdlColor::RGB(255, 165, 0),
        }
    }

    fn convert_color(&self, color: Color) -> SdlColor {
        SdlColor::RGB(color.r, color.g, color.b)
    }
}

impl Display for SdlDisplay {
    type Error = String;

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.canvas.set_draw_color(SdlColor::RGB(0, 0, 0));
        self.canvas.clear();
        Ok(())
    }

    fn draw_block(
        &mut self,
        position: Position,
        tetromino_type: TetrominoType,
    ) -> Result<(), Self::Error> {
        let color = self.tetromino_color(tetromino_type);
        self.canvas.set_draw_color(color);

        let rect = Rect::new(
            position.x * self.block_size_in_pixels as i32,
            position.y * self.block_size_in_pixels as i32,
            self.block_size_in_pixels,
            self.block_size_in_pixels,
        );

        self.canvas.fill_rect(rect).map_err(|e| e.to_string())?;

        self.canvas.set_draw_color(SdlColor::RGB(255, 255, 255));
        self.canvas.draw_rect(rect).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        color: Color,
    ) -> Result<(), Self::Error> {
        let sdl_color = self.convert_color(color);
        self.canvas.set_draw_color(sdl_color);

        let rect = Rect::new(x, y, width, height);
        self.canvas.draw_rect(rect).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn present(&mut self) -> Result<(), Self::Error> {
        self.canvas.present();
        Ok(())
    }
}
