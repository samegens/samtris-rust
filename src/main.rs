use crate::constants::*;
use crate::game::Game;
use crate::game_state::GameState;
use crate::game_timer::GameTimer;
use crate::graphics::SdlDisplay;
use crate::gui::Event;
use crate::gui::GameInput;
use crate::playfield::Playfield;
use common::Dimensions;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::EventPump;
use std::time::Duration;
use tetromino::RandomTetrominoGenerator;

mod common;
mod constants;
mod game;
mod game_state;
mod game_timer;
mod graphics;
mod gravity_timer;
mod gui;
mod playfield;
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
    let tetrominos_texture = texture_creator
        .load_texture("assets/blocks.png")
        .expect("Failed to load tetrominos texture");

    let mut display = SdlDisplay::new(canvas, BLOCK_SIZE, tetrominos_texture);

    let playfield = Playfield::new(playfield_dimensions);
    let mut game = Game::new(playfield, Box::new(RandomTetrominoGenerator::new()));
    game.spawn_tetromino();

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

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
