use crate::dimensions::Dimensions;
use crate::display::Display;
use crate::position::Position;
use crate::tetromino_type::TetrominoType;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct SdlDisplay {
    canvas: Canvas<Window>,
    block_size_in_pixels: u32,
    playfield_dimensions: Dimensions,
}

impl SdlDisplay {
    pub fn new(
        canvas: Canvas<Window>,
        block_size_in_pixels: u32,
        playfield_dimensions: Dimensions,
    ) -> Self {
        Self {
            canvas,
            block_size_in_pixels,
            playfield_dimensions,
        }
    }

    fn tetromino_color(&self, tetromino_type: TetrominoType) -> Color {
        match tetromino_type {
            TetrominoType::I => Color::RGB(0, 255, 255), // Cyan
            TetrominoType::O => Color::RGB(255, 255, 0), // Yellow
            TetrominoType::T => Color::RGB(128, 0, 128), // Purple
            TetrominoType::Z => Color::RGB(255, 0, 0),   // Red
            TetrominoType::S => Color::RGB(0, 255, 0),   // Green
            TetrominoType::J => Color::RGB(0, 0, 255),   // Blue
            TetrominoType::L => Color::RGB(255, 165, 0), // Orange
        }
    }
}

impl Display for SdlDisplay {
    type Error = String;

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
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

        // Draw border
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.draw_rect(rect).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn draw_playfield_border(&mut self) -> Result<(), Self::Error> {
        self.canvas.set_draw_color(Color::RGB(128, 128, 128));

        let playfield_width_pixels =
            self.playfield_dimensions.width as i32 * self.block_size_in_pixels as i32;
        let playfield_height_pixels =
            self.playfield_dimensions.height as i32 * self.block_size_in_pixels as i32;

        let border_rect = Rect::new(
            -1,                                   // Start 1 pixel to the left
            -1,                                   // Start 1 pixel above
            (playfield_width_pixels + 2) as u32,  // Width + 2 pixels for left/right borders
            (playfield_height_pixels + 2) as u32, // Height + 2 pixels for top/bottom borders
        );

        self.canvas
            .draw_rect(border_rect)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn present(&mut self) -> Result<(), Self::Error> {
        self.canvas.present();
        Ok(())
    }
}
