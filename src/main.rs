use crate::constants::*;
use crate::game_logic::GameState;
use crate::game_logic::GameTimer;
use crate::graphics::Display;
use crate::graphics::SdlDisplay;
use crate::gui::Event;
use crate::gui::GameInput;
use crate::screens::GameScreen;
use crate::screens::ScreenResult;
use common::Dimensions;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::EventPump;
use std::time::Duration;

mod animation;
mod common;
mod constants;
mod events;
mod game_logic;
mod graphics;
mod gui;
mod menu;
mod screens;
#[cfg(test)]
mod test_helpers;
mod tetromino;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let _image_context = image::init(InitFlag::PNG)?;

    let video_subsystem = sdl_context.video()?;
    let mut event_pump = sdl_context.event_pump()?;

    let window_width = WINDOW_WIDTH_IN_BLOCKS * BLOCK_SIZE;
    let window_height = WINDOW_HEIGHT_IN_BLOCKS * BLOCK_SIZE;

    let window = video_subsystem
        .window("SAMTris Rust", window_width, window_height)
        .position_centered()
        .build()?;
    let canvas = window.into_canvas().build()?;

    let texture_creator = canvas.texture_creator();
    let png_data = include_bytes!("../assets/blocks.png");
    let tetrominos_texture = texture_creator
        .load_texture_bytes(png_data)
        .expect("Failed to load embedded tetrominos texture");

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_data = include_bytes!("../assets/font.woff");
    let font = ttf_context.load_font_from_rwops(sdl2::rwops::RWops::from_bytes(font_data)?, 16)?;

    let mut display = SdlDisplay::new(canvas, BLOCK_SIZE, tetrominos_texture, font);

    let mut current_screen = GameScreen::new();
    let mut game_timer = GameTimer::new();

    'running: loop {
        let result = current_screen.handle_events(&mut event_pump);

        match result {
            ScreenResult::Continue => {}
            ScreenResult::Quit => break 'running,
            // TODO: Handle other screen transitions later
            _ => break 'running, // For now, treat others as quit
        }

        current_screen.update(game_timer.delta());
        display.clear()?;
        current_screen.draw(&mut display)?;
        display.present()?;

        // This is not completely accurate, but it helps to keep the game running at a reasonably consistent frame rate.
        // It doesn't account for the time taken to process events or draw the frame. For Tetris it's good enough.
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
