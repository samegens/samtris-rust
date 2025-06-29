use crate::common::Dimensions;
use crate::constants::*;
use crate::events::EventQueue;
use crate::game_logic::Game;
use crate::game_logic::GameState;
use crate::game_logic::GameTimer;
use crate::game_logic::Playfield;
use crate::graphics::Display;
use crate::graphics::GraphicsHudRenderer;
use crate::graphics::GraphicsPlayfieldRenderer;
use crate::gui::Event;
use crate::gui::GameInput;
use crate::screens::ScreenResult;
use crate::tetromino::RandomTetrominoGenerator;
use sdl2::EventPump;
use std::sync::Arc;
use std::time::Duration;

pub struct GameScreen {
    game: Game<GraphicsPlayfieldRenderer, GraphicsHudRenderer, RandomTetrominoGenerator>,
    game_timer: GameTimer,
}

impl GameScreen {
    pub fn new() -> Self {
        let playfield_dimensions = Dimensions::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);
        let event_bus = Arc::new(EventQueue::new());
        let playfield = Playfield::new(
            playfield_dimensions,
            RandomTetrominoGenerator::new(),
            event_bus.clone(),
        );
        let mut game = Game::new(
            playfield,
            GraphicsPlayfieldRenderer::new(),
            GraphicsHudRenderer::new(),
            event_bus.clone(),
        );
        game.start_level(0);

        Self {
            game,
            game_timer: GameTimer::new(),
        }
    }

    pub fn update(&mut self, _delta_time: Duration) {
        self.game.update(self.game_timer.delta());
    }

    pub fn handle_events(&mut self, event_pump: &mut EventPump) -> ScreenResult {
        let events = self.poll_events(event_pump);
        for event in events {
            match event {
                Event::Quit => return ScreenResult::Quit,
                Event::GameInput(game_input) => {
                    self.game.handle_input(game_input);
                }
            }
        }
        ScreenResult::Continue
    }

    pub fn draw<D: Display>(&mut self, display: &mut D) -> Result<(), String> {
        self.game.draw(display)
    }

    fn poll_events(&self, event_pump: &mut EventPump) -> Vec<Event> {
        let mut events = Vec::new();

        for sdl_event in event_pump.poll_iter() {
            match self.game.get_game_state() {
                GameState::Playing => {
                    self.handle_playing_events(&mut events, sdl_event);
                }
                GameState::GameOver => {
                    self.handle_game_over_events(&mut events, sdl_event);
                }
            }
        }

        events
    }

    fn handle_playing_events(&self, events: &mut Vec<Event>, sdl_event: sdl2::event::Event) {
        match sdl_event {
            sdl2::event::Event::Quit { .. } => {
                events.push(Event::Quit);
            }
            sdl2::event::Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                sdl2::keyboard::Keycode::Left => events.push(Event::GameInput(GameInput::MoveLeft)),
                sdl2::keyboard::Keycode::Right => {
                    events.push(Event::GameInput(GameInput::MoveRight))
                }
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

    fn handle_game_over_events(&self, events: &mut Vec<Event>, sdl_event: sdl2::event::Event) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::MockDisplay;

    #[test]
    fn game_screen_can_be_created() {
        // Act
        let sut = GameScreen::new();

        // Assert
        assert_eq!(*sut.game.get_game_state(), GameState::Playing);
    }

    #[test]
    fn game_screen_can_draw() {
        // Arrange
        let mut sut = GameScreen::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.cleared);
        assert!(display.presented);
    }
}
