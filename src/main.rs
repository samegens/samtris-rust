use crate::constants::*;
use crate::events::EventQueue;
use crate::game_logic::Game;
use crate::game_logic::GameState;
use crate::game_logic::GameTimer;
use crate::game_logic::Playfield;
use crate::graphics::GraphicsPlayfieldRenderer;
use crate::graphics::SdlDisplay;
use crate::gui::Event;
use crate::gui::GameInput;
use common::Dimensions;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::EventPump;
use std::sync::Arc;
use std::time::Duration;
use tetromino::RandomTetrominoGenerator;

mod animation;
mod common;
mod constants;
mod events;
mod game_logic;
mod graphics;
mod gui;
#[cfg(test)]
mod test_helpers;
mod tetromino;

fn poll_events(event_pump: &mut EventPump, game_state: &GameState) -> Vec<Event> {
    let mut events = Vec::new();

    for sdl_event in event_pump.poll_iter() {
        match game_state {
            GameState::Playing => {
                handle_playing_events(&mut events, sdl_event);
            }
            GameState::GameOver => {
                handle_game_over_events(&mut events, sdl_event);
            }
        }
    }

    events
}

fn handle_playing_events(events: &mut Vec<Event>, sdl_event: sdl2::event::Event) {
    match sdl_event {
        sdl2::event::Event::Quit { .. } => {
            events.push(Event::Quit);
        }
        sdl2::event::Event::KeyDown {
            keycode: Some(keycode),
            ..
        } => match keycode {
            sdl2::keyboard::Keycode::Left => events.push(Event::GameInput(GameInput::MoveLeft)),
            sdl2::keyboard::Keycode::Right => events.push(Event::GameInput(GameInput::MoveRight)),
            sdl2::keyboard::Keycode::Up | sdl2::keyboard::Keycode::X => {
                events.push(Event::GameInput(GameInput::RotateClockwise))
            }
            sdl2::keyboard::Keycode::Down => events.push(Event::GameInput(GameInput::MoveDown)),
            sdl2::keyboard::Keycode::Z => {
                events.push(Event::GameInput(GameInput::RotateCounterclockwise))
            }
            sdl2::keyboard::Keycode::Space => {
                events.push(Event::GameInput(GameInput::Drop));
            }
            sdl2::keyboard::Keycode::Escape => {
                events.push(Event::Quit);
            }
            _ => {}
        },
        _ => {}
    }
}

fn handle_game_over_events(events: &mut Vec<Event>, sdl_event: sdl2::event::Event) {
    match sdl_event {
        sdl2::event::Event::Quit { .. } => {
            events.push(Event::Quit);
        }
        sdl2::event::Event::KeyDown {
            keycode:
                Some(
                    sdl2::keyboard::Keycode::Space
                    | sdl2::keyboard::Keycode::Return
                    | sdl2::keyboard::Keycode::Escape,
                ),
            ..
        } => {
            events.push(Event::GameInput(GameInput::StartGame));
        }
        _ => {}
    }
}

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
    let png_data = include_bytes!("../assets/blocks.png");
    let tetrominos_texture = texture_creator
        .load_texture_bytes(png_data)
        .expect("Failed to load embedded tetrominos texture");

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_data = include_bytes!("../assets/font.woff");
    let font = ttf_context.load_font_from_rwops(sdl2::rwops::RWops::from_bytes(font_data)?, 16)?;

    let mut display = SdlDisplay::new(canvas, BLOCK_SIZE, tetrominos_texture, font);

    let event_bus = Arc::new(EventQueue::new());
    let playfield = Playfield::new(
        playfield_dimensions,
        RandomTetrominoGenerator::new(),
        event_bus.clone(),
    );
    let mut game = Game::new(
        playfield,
        GraphicsPlayfieldRenderer::new(),
        event_bus.clone(),
    );
    game.start_level(0);

    let mut game_timer = GameTimer::new();

    'running: loop {
        game.update(game_timer.delta());

        let events = poll_events(&mut event_pump, game.get_game_state());
        for event in events {
            match event {
                Event::Quit => break 'running,
                Event::GameInput(game_input) => {
                    game.handle_input(game_input);
                }
            }
        }

        game.draw(&mut display)?;

        // This is not completely accurate, but it helps to keep the game running at a reasonably consistent frame rate.
        // It doesn't account for the time taken to process events or draw the frame. For Tetris it's good enough.
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
