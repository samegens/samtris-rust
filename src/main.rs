use crate::common::Position;
use crate::constants::{
    BLOCK_SIZE, PLAYFIELD_BORDER_WIDTH, PLAYFIELD_HEIGHT, PLAYFIELD_OFFSET_X, PLAYFIELD_OFFSET_Y,
    PLAYFIELD_WIDTH, WINDOW_HEIGHT_IN_BLOCKS, WINDOW_WIDTH_IN_BLOCKS,
};
use crate::game::Game;
use crate::game_input::GameInput;
use crate::graphics::Color;
use crate::graphics::Display;
use crate::graphics::SdlDisplay;
use crate::playfield::Playfield;
use crate::tetromino_type::TetrominoType;
use common::Dimensions;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod common;
mod constants;
mod game;
mod game_input;
mod graphics;
mod playfield;
mod tetromino_definition;
mod tetromino_definitions;
mod tetromino_instance;
mod tetromino_type;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playfield_dimensions = Dimensions::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);

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
    let tetrominos_texture = texture_creator
        .load_texture("assets/blocks.png")
        .expect("Failed to load tetrominos texture");

    let mut display = SdlDisplay::new(canvas, BLOCK_SIZE, tetrominos_texture);

    let playfield = Playfield::new(playfield_dimensions);
    let mut game = Game::new(playfield);
    game.spawn_tetromino(TetrominoType::T);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Left => {
                        game.handle_input(GameInput::MoveLeft);
                    }
                    Keycode::Right => {
                        game.handle_input(GameInput::MoveRight);
                    }
                    Keycode::Up => {
                        game.handle_input(GameInput::RotateClockwise);
                    }
                    Keycode::Down => {
                        game.handle_input(GameInput::MoveDown);
                    }
                    Keycode::Escape => break 'running,
                    _ => {}
                },
                _ => {}
            }
        }

        display
            .clear()
            .map_err(|e| format!("Clear failed: {}", e))?;

        // Draw current tetromino directly
        let playfield_position =
            Position::new(PLAYFIELD_OFFSET_X as i32, PLAYFIELD_OFFSET_Y as i32);
        if let Some(tetromino) = game.get_current_tetromino() {
            let blocks = tetromino.get_world_blocks();
            let tetromino_type = tetromino.get_type();

            for position in blocks {
                let window_position = playfield_position + position.scale(BLOCK_SIZE as i32);
                display
                    .draw_block(window_position, tetromino_type)
                    .map_err(|e| format!("Draw block failed: {}", e))?;
            }
        }

        // Draw playfield border
        let border_color = Color::WHITE;
        display
            .draw_rectangle(
                PLAYFIELD_OFFSET_X - PLAYFIELD_BORDER_WIDTH,
                PLAYFIELD_OFFSET_Y,
                PLAYFIELD_BORDER_WIDTH,
                PLAYFIELD_HEIGHT * BLOCK_SIZE,
                border_color,
            )
            .map_err(|e| format!("Draw border left failed: {}", e))?;
        display
            .draw_rectangle(
                PLAYFIELD_OFFSET_X - PLAYFIELD_BORDER_WIDTH,
                PLAYFIELD_OFFSET_Y + PLAYFIELD_HEIGHT * BLOCK_SIZE,
                PLAYFIELD_BORDER_WIDTH + PLAYFIELD_WIDTH * BLOCK_SIZE + PLAYFIELD_BORDER_WIDTH,
                PLAYFIELD_BORDER_WIDTH,
                border_color,
            )
            .map_err(|e| format!("Draw border bottom failed: {}", e))?;
        display
            .draw_rectangle(
                PLAYFIELD_OFFSET_X + PLAYFIELD_WIDTH * BLOCK_SIZE,
                PLAYFIELD_OFFSET_Y,
                PLAYFIELD_BORDER_WIDTH,
                PLAYFIELD_HEIGHT * BLOCK_SIZE,
                border_color,
            )
            .map_err(|e| format!("Draw border right failed: {}", e))?;

        // Present frame
        display
            .present()
            .map_err(|e| format!("Present failed: {}", e))?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
