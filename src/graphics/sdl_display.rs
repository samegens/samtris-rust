// src/sdl_display.rs
use crate::common::Position;
use crate::graphics::Color;
use crate::graphics::Display;
use crate::tetromino::TetrominoType;
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::ttf::Font;
use sdl2::video::Window;

pub struct SdlDisplay<'a> {
    canvas: Canvas<Window>,
    block_size_in_pixels: u32,
    tetrominos_texture: Texture<'a>,
    font: Font<'a, 'a>,
}

impl<'a> SdlDisplay<'a> {
    pub fn new(
        canvas: Canvas<Window>,
        block_size_in_pixels: u32,
        tetrominos_texture: Texture<'a>,
        font: Font<'a, 'a>,
    ) -> Self {
        Self {
            canvas,
            block_size_in_pixels,
            tetrominos_texture,
            font,
        }
    }

    fn convert_color(&self, color: Color) -> SdlColor {
        SdlColor::RGB(color.r, color.g, color.b)
    }

    fn get_blocks_texture_rect(&self, tetromino_type: TetrominoType) -> Rect {
        let index = tetromino_type as i32;
        const TEXTURE_BLOCK_SIZE: u32 = 16;
        Rect::new(
            index * TEXTURE_BLOCK_SIZE as i32,
            0,
            TEXTURE_BLOCK_SIZE,
            TEXTURE_BLOCK_SIZE,
        )
    }
}

impl<'a> Display for SdlDisplay<'a> {
    fn clear(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(SdlColor::RGB(0, 0, 0));
        self.canvas.clear();
        Ok(())
    }

    fn draw_block(
        &mut self,
        position: Position,
        tetromino_type: TetrominoType,
    ) -> Result<(), String> {
        let src_rect = self.get_blocks_texture_rect(tetromino_type);

        let dst_rect = Rect::new(
            position.x,
            position.y,
            self.block_size_in_pixels,
            self.block_size_in_pixels,
        );

        self.canvas
            .copy(&self.tetrominos_texture, Some(src_rect), Some(dst_rect))
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn draw_rectangle(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: Color,
    ) -> Result<(), String> {
        let sdl_color = self.convert_color(color);
        self.canvas.set_draw_color(sdl_color);

        let rect = Rect::new(x as i32, y as i32, width, height);
        self.canvas.fill_rect(rect).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn draw_text(&mut self, text: &str, x: u32, y: u32, color: Color) -> Result<(), String> {
        let sdl_color = self.convert_color(color);
        let surface = self
            .font
            .render(text)
            .blended(sdl_color)
            .map_err(|e| e.to_string())?;

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let target = Rect::new(x as i32, y as i32, surface.width(), surface.height());
        self.canvas
            .copy(&texture, None, Some(target))
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn present(&mut self) -> Result<(), String> {
        self.canvas.present();
        Ok(())
    }
}
