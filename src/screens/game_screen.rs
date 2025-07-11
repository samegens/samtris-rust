use crate::common::Dimensions;
use crate::constants::*;
use crate::events::EventQueue;
use crate::game_logic::Game;
use crate::game_logic::GameResult;
use crate::game_logic::GameState;
use crate::game_logic::Playfield;
use crate::graphics::Display;
use crate::graphics::GraphicsHudRenderer;
use crate::graphics::GraphicsPlayfieldRenderer;
use crate::gui::GameInput;
use crate::high_scores::HighScoreManager;
#[cfg(test)]
use crate::high_scores::MockHighScoresRepository;
use crate::input::InputEvent;
use crate::input::Key;
use crate::screens::Screen;
use crate::screens::ScreenResult;
use crate::tetromino::RandomTetrominoGenerator;
use std::sync::Arc;
use std::time::Duration;

pub struct GameScreen {
    game: Game<GraphicsPlayfieldRenderer, GraphicsHudRenderer, RandomTetrominoGenerator>,
}

impl GameScreen {
    pub fn new(high_score_manager: HighScoreManager) -> Self {
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
            high_score_manager,
        );
        game.start_level(0);

        Self { game }
    }

    fn translate_key_to_game_input(&self, key: Key) -> Option<GameInput> {
        match self.game.get_game_state() {
            GameState::Playing => match key {
                Key::Left => Some(GameInput::MoveLeft),
                Key::Right => Some(GameInput::MoveRight),
                Key::Down => Some(GameInput::MoveDown),
                Key::Up | Key::Alphanumeric('X') => Some(GameInput::RotateClockwise),
                Key::Alphanumeric('Z') => Some(GameInput::RotateCounterclockwise),
                Key::Space => Some(GameInput::Drop),
                _ => None,
            },
            GameState::GameOver => match key {
                Key::Space | Key::Enter => Some(GameInput::StartGame),
                _ => None,
            },
            _ => None,
        }
    }

    fn handle_game_input(&mut self, key: Key) -> ScreenResult {
        if let Some(game_input) = self.translate_key_to_game_input(key) {
            match self.game.handle_input(game_input) {
                GameState::Playing => {
                    // Continue playing, no special action needed
                }
                GameState::GameOver => {
                    // Continue showing game over, no special action needed
                }
                GameState::ReturnToMainMenu => {
                    return ScreenResult::ReturnToMainMenu;
                }
                GameState::EnterHighScore(level, score) => {
                    return ScreenResult::EnterHighScore(GameResult { level, score });
                }
            }
        }

        ScreenResult::Continue
    }

    #[cfg(test)]
    pub fn get_game(
        &self,
    ) -> &Game<GraphicsPlayfieldRenderer, GraphicsHudRenderer, RandomTetrominoGenerator> {
        &self.game
    }
}

impl Screen for GameScreen {
    fn update(&mut self, delta_time: Duration) {
        self.game.update(delta_time);
    }

    fn draw(&mut self, display: &mut dyn Display) -> Result<(), String> {
        self.game.draw(display)
    }

    fn handle_input(&mut self, input_events: &[InputEvent]) -> ScreenResult {
        for event in input_events {
            match event {
                InputEvent::Quit => return ScreenResult::Quit,
                InputEvent::KeyPressed(Key::Escape) => return ScreenResult::ReturnToMainMenu,
                InputEvent::KeyPressed(key) => {
                    let result = self.handle_game_input(*key);
                    if result != ScreenResult::Continue {
                        return result;
                    }
                }
            }
        }

        ScreenResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::{graphics::MockDisplay, test_helpers::get_tetromino_position_from_gamescreen};

    #[test]
    fn game_screen_can_be_created() {
        // Act
        let sut = create_test_game_screen();

        // Assert
        assert_eq!(*sut.game.get_game_state(), GameState::Playing);
    }

    #[test]
    fn game_screen_can_draw() {
        // Arrange
        let mut sut = create_test_game_screen();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.cleared);
        assert!(display.presented);
    }

    #[test]
    fn update_advances_game_time() {
        // Arrange
        let mut sut = create_test_game_screen();
        sut.game.spawn_tetromino();
        let initial_position = get_tetromino_position_from_gamescreen(&sut);

        // Act
        sut.update(Duration::from_millis(1000));

        // Assert
        let new_position = get_tetromino_position_from_gamescreen(&sut);
        assert_eq!(new_position.y, initial_position.y + 1);
    }

    #[test]
    fn handle_input_moves_tetromino_left() {
        // Arrange
        let mut sut = create_test_game_screen();
        sut.game.spawn_tetromino();
        let initial_position = get_tetromino_position_from_gamescreen(&sut);
        let input_events = vec![InputEvent::KeyPressed(Key::Left)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Continue);
        let new_position = get_tetromino_position_from_gamescreen(&sut);
        assert_eq!(new_position.x, initial_position.x - 1);
        assert_eq!(new_position.y, initial_position.y);
    }

    #[test]
    fn handle_input_quit_returns_quit_screen_result() {
        // Arrange
        let mut sut = create_test_game_screen();
        let input_events = vec![InputEvent::Quit];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Quit);
    }

    #[rstest]
    #[case(Key::Left, Some(GameInput::MoveLeft))]
    #[case(Key::Right, Some(GameInput::MoveRight))]
    #[case(Key::Down, Some(GameInput::MoveDown))]
    #[case(Key::Up, Some(GameInput::RotateClockwise))]
    #[case(Key::Alphanumeric('X'), Some(GameInput::RotateClockwise))]
    #[case(Key::Alphanumeric('Z'), Some(GameInput::RotateCounterclockwise))]
    #[case(Key::Space, Some(GameInput::Drop))]
    #[case(Key::Enter, None)]
    #[case(Key::Escape, None)] // Handled separately in handle_input
    fn translate_key_to_game_input_when_playing(
        #[case] key: Key,
        #[case] expected: Option<GameInput>,
    ) {
        // Arrange
        let sut = create_test_game_screen(); // Starts in Playing state

        // Act
        let result = sut.translate_key_to_game_input(key);

        // Assert
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(Key::Space, Some(GameInput::StartGame))]
    #[case(Key::Enter, Some(GameInput::StartGame))]
    #[case(Key::Escape, None)] // Handled separately in handle_input
    #[case(Key::Left, None)]
    #[case(Key::Right, None)]
    #[case(Key::Down, None)]
    #[case(Key::Up, None)]
    #[case(Key::Alphanumeric('Z'), None)]
    #[case(Key::Alphanumeric('X'), None)]
    fn translate_key_to_game_input_when_game_over(
        #[case] key: Key,
        #[case] expected: Option<GameInput>,
    ) {
        // Arrange
        let mut sut = create_test_game_screen();
        sut.game.set_game_state_game_over();

        // Act
        let result = sut.translate_key_to_game_input(key);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn handle_input_escape_returns_to_main_menu() {
        // Arrange
        let mut sut = create_test_game_screen();
        let input_events = vec![InputEvent::KeyPressed(Key::Escape)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::ReturnToMainMenu);
    }

    fn create_test_game_screen() -> GameScreen {
        let high_score_manager = HighScoreManager::new(Box::new(MockHighScoresRepository::empty()));
        GameScreen::new(high_score_manager)
    }
}
