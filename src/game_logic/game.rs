use crate::constants::*;
use crate::events::{Event, EventBus, EventType};
use crate::game_logic::GameState;
use crate::game_logic::{Playfield, PlayfieldState};
use crate::graphics::{Color, Display, PlayfieldRenderer};
use crate::gui::GameInput;
use crate::tetromino::TetrominoGenerator;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Game<R: PlayfieldRenderer, T: TetrominoGenerator> {
    playfield: Playfield<T>,
    playfield_renderer: R,
    game_state: GameState,
    level: Arc<Mutex<u32>>,
}

impl<R: PlayfieldRenderer, T: TetrominoGenerator> Game<R, T> {
    pub fn new(playfield: Playfield<T>, playfield_renderer: R, event_bus: Arc<EventBus>) -> Self {
        let level = Arc::new(Mutex::new(0u32));
        let level_clone = level.clone();

        event_bus.subscribe(EventType::LevelStarted, move |event| {
            let Event::LevelStarted(level) = event;
            *level_clone.lock().unwrap() = *level;
        });
        Self {
            playfield,
            playfield_renderer,
            game_state: GameState::Playing,
            level,
        }
    }

    #[cfg(test)]
    pub fn get_playfield(&self) -> &Playfield<T> {
        &self.playfield
    }

    #[cfg(test)]
    pub fn get_playfield_mut(&mut self) -> &mut Playfield<T> {
        &mut self.playfield
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }

    pub fn spawn_tetromino(&mut self) -> bool {
        if self.playfield.spawn_tetromino() == PlayfieldState::GameOver {
            self.game_state = GameState::GameOver;
            return false;
        }

        true
    }

    /// Handle game input, returns true if the tetromino was moved successfully, false otherwise.
    pub fn handle_input(&mut self, input: GameInput) {
        match self.game_state {
            GameState::Playing => {
                let _ = self.playfield.handle_input(input);
            }
            GameState::GameOver => self.handle_game_over_input(input),
        };
    }

    pub fn handle_game_over_input(&mut self, input: GameInput) {
        if input == GameInput::StartGame {
            self.start_game();
        }
    }

    pub fn draw<D: Display>(&mut self, display: &mut D) -> Result<(), String> {
        display.clear()?;

        let playfield_view = self.playfield.get_view();

        self.playfield_renderer.draw(
            &playfield_view,
            &playfield_view.full_lines,
            playfield_view.show_blinking_lines,
            // TODO: remove blinking_lines and playfield_view, retrieve them from the view
            display,
        )?;

        self.draw_level(display)?;

        if self.game_state == GameState::GameOver {
            self.draw_game_over(display)?;
        }

        display.present()?;

        Ok(())
    }

    fn draw_level<D: Display>(&self, display: &mut D) -> Result<(), String> {
        
    }

    pub fn draw_game_over<D: Display>(&self, display: &mut D) -> Result<(), String> {
        let width = 100;
        let height = 50;
        let x = PLAYFIELD_OFFSET_X + (PLAYFIELD_WIDTH * BLOCK_SIZE - width) / 2;
        let y = PLAYFIELD_OFFSET_Y + (PLAYFIELD_HEIGHT * BLOCK_SIZE - height) / 2;

        display.draw_rectangle(x, y, width, height, Color::RED)?;

        Ok(())
    }

    pub fn update(&mut self, delta_time: Duration) {
        if let GameState::Playing = self.game_state {
            if self.playfield.update(delta_time) == PlayfieldState::GameOver {
                self.game_state = GameState::GameOver;
            }
        }
    }

    fn start_game(&mut self) {
        self.playfield.clear();
        self.spawn_tetromino();
        self.game_state = GameState::Playing;
    }

    #[cfg(test)]
    pub fn set_game_state_game_over(&mut self) {
        self.game_state = GameState::GameOver;
    }

    pub fn start_level(&mut self, level: u32) {
        self.playfield.start_level(level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Dimensions, Position};
    use crate::graphics::MockDisplay;
    use crate::gui::game_input::GameInput;
    use crate::test_helpers::*;
    use crate::tetromino::TetrominoDefinitions;
    use crate::tetromino::{TetrominoInstance, TetrominoType};
    use std::time::Duration;

    #[test]
    fn new_game_has_no_current_tetromino() {
        // Arrange
        let sut = create_standard_test_game();

        // Act
        let result: Option<&TetrominoInstance> = {
            let this = &sut;
            this.playfield.get_current_tetromino()
        };

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn new_game_has_game_state_playing() {
        // Arrange
        let sut = create_standard_test_game();

        // Act
        let result: &GameState = sut.get_game_state();

        // Assert
        let expected_game_state = GameState::Playing;
        assert_eq!(result, &expected_game_state);
    }

    #[test]
    fn tetromino_falls_automatically_when_gravity_timer_expires() {
        // Arrange
        let mut sut = create_test_game(TetrominoType::I);
        sut.spawn_tetromino();
        let initial_position = {
            let this = &sut;
            this.playfield.get_current_tetromino()
        }
        .unwrap()
        .get_position();

        // Act
        sut.update(Duration::from_millis(1000));

        // Assert
        let new_position = {
            let this = &sut;
            this.playfield.get_current_tetromino()
        }
        .unwrap()
        .get_position();
        assert_eq!(new_position.y, initial_position.y + 1);
    }

    #[test]
    fn tetromino_does_not_fall_automatically_before_gravity_timer_expires() {
        // Arrange
        let mut sut = create_test_game(TetrominoType::I);
        sut.spawn_tetromino();
        let initial_position = {
            let this = &sut;
            this.playfield.get_current_tetromino()
        }
        .unwrap()
        .get_position();

        // Act
        sut.update(Duration::from_millis(500));

        // Assert
        let new_position = {
            let this = &sut;
            this.playfield.get_current_tetromino()
        }
        .unwrap()
        .get_position();
        assert_eq!(new_position.y, initial_position.y);
    }

    #[test]
    fn tetromino_locks_when_gravity_cannot_move_it_down() {
        // Arrange
        let dimensions = Dimensions::new(PLAYFIELD_WIDTH, 5);
        let mut playfield = create_test_playfield_with_dimensions(dimensions);
        let definitions = TetrominoDefinitions::new();

        // Place an O tetromino directly below spawn position to block movement
        let blocking_tetromino = TetrominoInstance::new(
            TetrominoType::O,
            Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y + 2),
            &definitions,
        );
        playfield.set_current_tetromino(Some(blocking_tetromino));
        playfield.lock_tetromino();

        let mut sut = create_test_game_with_playfield(playfield);
        sut.spawn_tetromino();

        // Act
        sut.update(Duration::from_millis(1000)); // Trigger gravity

        // Assert
        assert!(sut
            .get_playfield()
            .is_position_occupied(Position::new(TETRIS_SPAWN_X + 1, TETRIS_SPAWN_Y + 1)));
    }

    #[test]
    fn draw_coordinates_display_lifecycle() {
        // Arrange
        let mut sut = create_standard_test_game();
        sut.spawn_tetromino();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.cleared);
        assert!(display.presented);
        let draw_calls = sut.playfield_renderer.get_draw_calls();
        assert_eq!(draw_calls.len(), 1);
    }

    #[test]
    fn start_game_clears_playfield_spawns_tetromino_and_sets_playing_state() {
        // Arrange
        let mut sut = create_test_game(TetrominoType::O);
        lock_tetromino(
            &mut sut,
            TetrominoType::O,
            Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y),
        );
        sut.set_game_state_game_over();

        // Act
        sut.handle_input(GameInput::StartGame);

        // Assert
        assert!(!sut
            .playfield
            .is_position_occupied(Position::new(TETRIS_SPAWN_X + 1, TETRIS_SPAWN_Y + 1)));
        assert_eq!(sut.game_state, GameState::Playing);
        assert!(sut.playfield.get_current_tetromino().is_some());
    }

    #[test]
    fn draw_when_game_over_draws_game_over_rectangle() {
        // Arrange
        let mut sut = create_test_game(TetrominoType::O);
        sut.set_game_state_game_over();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display
            .drawn_rectangles
            .iter()
            .any(|(_, _, _, _, color)| *color == Color::RED));
    }

    #[test]
    fn update_when_game_over_does_not_apply_gravity() {
        // Arrange
        let mut sut = create_test_game(TetrominoType::O);
        sut.spawn_tetromino();
        let expected = sut
            .playfield
            .get_current_tetromino()
            .as_ref()
            .unwrap()
            .get_position();
        sut.set_game_state_game_over();

        // Act
        sut.update(Duration::from_millis(1000));

        // Assert
        let actual = sut
            .playfield
            .get_current_tetromino()
            .as_ref()
            .unwrap()
            .get_position();
        assert_eq!(actual, expected);
    }

    #[test]
    fn draw_when_animating_lines_passes_correct_blink_state_to_renderer() {
        // Arrange
        let mut sut = create_standard_test_game();

        sut.playfield.set_state(PlayfieldState::AnimatingLines {
            countdown: Duration::from_millis(300),
            full_lines: vec![18, 19],
        });

        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());

        let draw_calls = sut.playfield_renderer.get_draw_calls();
        assert_eq!(draw_calls.len(), 1);

        let last_call = &draw_calls[0];
        assert!(last_call.show_blinking_lines); // Should be true for 300ms
        assert_eq!(last_call.blinking_lines, vec![18, 19]); // Should pass the filled lines
    }

    #[test]
    fn update_when_game_over_does_nothing() {
        // Arrange
        let mut sut = create_test_game(TetrominoType::O);
        sut.spawn_tetromino();
        let initial_position = sut
            .playfield
            .get_current_tetromino()
            .as_ref()
            .unwrap()
            .get_position();
        sut.game_state = GameState::GameOver;

        // Act
        sut.update(Duration::from_millis(1000));

        // Assert
        assert_eq!(sut.game_state, GameState::GameOver);
        let current_position = sut
            .playfield
            .get_current_tetromino()
            .as_ref()
            .unwrap()
            .get_position();
        assert_eq!(current_position, initial_position); // Tetromino didn't move
    }

    #[test]
    fn game_over_from_playfield_results_in_game_state_game_over() {
        // Arrange
        let mut sut = create_test_game(TetrominoType::O);
        sut.spawn_tetromino();
        sut.playfield.set_state(PlayfieldState::GameOver);

        // Act
        sut.update(Duration::from_millis(1000));

        // Assert
        assert_eq!(sut.game_state, GameState::GameOver);
    }
}
