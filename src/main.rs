use crate::constants::{TETRIS_PLAYFIELD_HEIGHT, TETRIS_PLAYFIELD_WIDTH};
use crate::dimensions::Dimensions;
use crate::display::Display;
use crate::game::Game;
use crate::game_input::GameInput;
use crate::playfield::Playfield;
use crate::sdl_display::SdlDisplay;
use crate::tetromino_type::TetrominoType;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod color;
mod constants;
mod dimensions;
mod display;
mod game;
mod game_input;
mod playfield;
mod position;
mod rotation_index;
mod sdl_display;
mod tetromino_definition;
mod tetromino_definitions;
mod tetromino_instance;
mod tetromino_type;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const BLOCK_SIZE: usize = 32;
    const PLAYFIELD_HORIZONTAL_MARGIN: usize = 100;
    const PLAYFIELD_VERTICAL_MARGIN: usize = 100;

    let playfield_dimensions = Dimensions::new(TETRIS_PLAYFIELD_WIDTH, TETRIS_PLAYFIELD_HEIGHT);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut event_pump = sdl_context.event_pump()?;

    let window_width = TETRIS_PLAYFIELD_WIDTH * BLOCK_SIZE + PLAYFIELD_HORIZONTAL_MARGIN;
    let window_height = TETRIS_PLAYFIELD_HEIGHT * BLOCK_SIZE + PLAYFIELD_VERTICAL_MARGIN;

    let window = video_subsystem
        .window("Tetris", window_width as u32, window_height as u32)
        .position_centered()
        .build()?;
    let canvas = window.into_canvas().build()?;
    let mut display = SdlDisplay::new(canvas, 30, playfield_dimensions);

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
        if let Some(tetromino) = game.get_current_tetromino() {
            let blocks = tetromino.get_world_blocks();
            let tetromino_type = tetromino.get_type();

            for position in blocks {
                display
                    .draw_block(position, tetromino_type)
                    .map_err(|e| format!("Draw block failed: {}", e))?;
            }
        }

        // Draw playfield border
        display
            .draw_playfield_border()
            .map_err(|e| format!("Draw border failed: {}", e))?;

        // Present frame
        display
            .present()
            .map_err(|e| format!("Present failed: {}", e))?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
