use crate::common::Dimensions;
use crate::constants::*;
use crate::events::EventQueue;
use crate::game_logic::Game;
use crate::game_logic::GameState;
use crate::game_logic::Playfield;
use crate::graphics::Display;
use crate::graphics::GraphicsHudRenderer;
use crate::graphics::GraphicsPlayfieldRenderer;
use crate::gui::GameInput;
use crate::input::InputEvent;
use crate::input::Key;
use crate::screens::ScreenResult;
use crate::tetromino::RandomTetrominoGenerator;
use std::sync::Arc;
use std::time::Duration;

pub struct GameScreen {
    game: Game<GraphicsPlayfieldRenderer, GraphicsHudRenderer, RandomTetrominoGenerator>,
}

impl GameScreen {
    pub fn new() -> Self {
        let playfield_dimensions = Dimensions::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);
        let event_queue = Arc::new(EventQueue::new());
        let playfield = Playfield::new(
            playfield_dimensions,
            RandomTetrominoGenerator::new(),
            event_queue.clone(),
        );
        let mut game = Game::new(
            playfield,
            GraphicsPlayfieldRenderer::new(),
            GraphicsHudRenderer::new(),
            event_queue.clone(),
        );
        game.start_level(0);

        Self { game }
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.game.update(delta_time);
    }

    pub fn draw<D: Display>(&mut self, display: &mut D) -> Result<(), String> {
        self.game.draw(display)
    }

    pub fn handle_input(&mut self, input_events: &[InputEvent]) -> ScreenResult {
        for event in input_events {
            match event {
                InputEvent::Quit => return ScreenResult::Quit,
                InputEvent::KeyPressed(key) => {
                    if let Some(game_input) = self.translate_key_to_game_input(*key) {
                        self.game.handle_input(game_input);
                    }
                }
                InputEvent::KeyReleased(_) => {
                    // Game doesn't currently need key release events
                }
            }
        }
        ScreenResult::Continue
    }

    fn translate_key_to_game_input(&self, key: Key) -> Option<GameInput> {
        match self.game.get_game_state() {
            GameState::Playing => {
                match key {
                    Key::Left => Some(GameInput::MoveLeft),
                    Key::Right => Some(GameInput::MoveRight),
                    Key::Down => Some(GameInput::MoveDown),
                    Key::Up | Key::X => Some(GameInput::RotateClockwise),
                    Key::Z => Some(GameInput::RotateCounterclockwise),
                    Key::Space => Some(GameInput::Drop),
                    Key::Escape => Some(GameInput::StartGame), // TODO: Should go back to menu
                    _ => None,
                }
            }
            GameState::GameOver => {
                match key {
                    Key::Space | Key::Enter => Some(GameInput::StartGame),
                    Key::Escape => Some(GameInput::StartGame), // TODO: Should go back to menu
                    _ => None,
                }
            }
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
