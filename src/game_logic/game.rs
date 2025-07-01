use crate::events::{Event, EventQueue};
use crate::game_logic::{GameState, LevelManager};
use crate::game_logic::{Playfield, PlayfieldState};
use crate::graphics::{Display, HudRenderer, HudView, PlayfieldRenderer};
use crate::gui::GameInput;
use crate::high_scores::HighScoreManager;
use crate::tetromino::TetrominoGenerator;
use std::sync::Arc;
use std::time::Duration;

pub struct Game<R: PlayfieldRenderer, H: HudRenderer, T: TetrominoGenerator> {
    event_queue: Arc<EventQueue>,
    playfield: Playfield<T>,
    playfield_renderer: R,
    hud_renderer: H,
    game_state: GameState,
    level_manager: LevelManager,
    high_score_manager: HighScoreManager,
}

impl<R: PlayfieldRenderer, H: HudRenderer, T: TetrominoGenerator> Game<R, H, T> {
    pub fn new(
        playfield: Playfield<T>,
        playfield_renderer: R,
        hud_renderer: H,
        event_queue: Arc<EventQueue>,
        high_score_manager: HighScoreManager,
    ) -> Self {
        let level_manager = LevelManager::new(event_queue.clone());

        Self {
            event_queue,
            playfield,
            playfield_renderer,
            hud_renderer,
            game_state: GameState::Playing,
            level_manager,
            high_score_manager,
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

    pub fn draw(&mut self, display: &mut dyn Display) -> Result<(), String> {
        display.clear()?;

        let playfield_view = self.playfield.get_view();

        self.playfield_renderer.draw(&playfield_view, display)?;

        let hud_view = self.get_hud_view();
        self.hud_renderer.draw(&hud_view, display)?;

        display.present()?;

        Ok(())
    }

    pub fn update(&mut self, delta_time: Duration) {
        self.process_event_queue();

        if let GameState::Playing = self.game_state {
            if self.playfield.update(delta_time) == PlayfieldState::GameOver {
                self.game_state = GameState::GameOver;
            }
        }
    }

    fn process_event_queue(&mut self) {
        let events = self.event_queue.drain();
        for event in events {
            self.handle_event(event);
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
        self.level_manager.start_level(level);
        self.playfield.start_level(level);
    }

    fn handle_event(&mut self, event: crate::events::Event) {
        match event {
            Event::LinesCleared(nr_lines) => {
                self.level_manager.handle_lines_cleared(nr_lines);
            }
            Event::LevelStarted(level) => {
                self.playfield.start_level(level);
            }
        }
    }

    fn get_hud_view(&self) -> crate::graphics::HudView {
        let playfield_view = self.playfield.get_view();
        HudView {
            next_tetromino_type: playfield_view.next_tetromino_type,
            current_level: self.level_manager.get_current_level(),
            total_lines_cleared: self.level_manager.get_total_lines_cleared(),
            score: self.level_manager.get_score(),
            show_game_over: self.game_state == GameState::GameOver,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{Dimensions, Position};
    use crate::constants::*;
    use crate::graphics::{MockDisplay, MockHudRenderer, MockPlayfieldRenderer};
    use crate::gui::GameInput;
    use crate::high_scores::MockHighScoresRepository;
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

    #[test]
    fn handle_input_when_playing_forwards_to_playfield() {
        // Arrange
        let mut sut = create_test_game(TetrominoType::O);
        sut.spawn_tetromino();
        let initial_position = sut
            .get_playfield()
            .get_current_tetromino()
            .unwrap()
            .get_position();

        // Act
        sut.handle_input(GameInput::MoveLeft);

        // Assert
        let new_position = sut
            .get_playfield()
            .get_current_tetromino()
            .unwrap()
            .get_position();
        assert_eq!(new_position.x, initial_position.x - 1);
        assert_eq!(new_position.y, initial_position.y);
    }

    #[test]
    fn process_event_queue_handles_lines_cleared_event() {
        // Arrange
        let event_queue = Arc::new(EventQueue::new());
        let playfield = create_test_playfield_with_event_queue(event_queue.clone());
        let high_score_manager = HighScoreManager::new(Box::new(MockHighScoresRepository::empty()));
        let mut sut = Game::new(
            playfield,
            MockPlayfieldRenderer::new(),
            MockHudRenderer::new(),
            event_queue.clone(),
            high_score_manager,
        );

        event_queue.push_back(Event::LinesCleared(4));
        let initial_lines = sut.level_manager.get_total_lines_cleared();

        // Act
        sut.update(Duration::from_millis(1));

        // Assert
        assert_eq!(
            sut.level_manager.get_total_lines_cleared(),
            initial_lines + 4
        );
    }

    #[test]
    fn process_event_queue_handles_level_started_event() {
        // Arrange
        let event_queue = Arc::new(EventQueue::new());
        let playfield = create_test_playfield_with_event_queue(event_queue.clone());
        let high_score_manager = HighScoreManager::new(Box::new(MockHighScoresRepository::empty()));
        let mut sut = Game::new(
            playfield,
            MockPlayfieldRenderer::new(),
            MockHudRenderer::new(),
            event_queue.clone(),
            high_score_manager,
        );
        sut.spawn_tetromino();

        event_queue.push_back(Event::LevelStarted(3));

        // Act
        sut.update(Duration::from_millis(1));

        // Assert
        assert_eq!(sut.playfield.get_gravity_timer().get_level(), 3);
    }

    #[test]
    fn draw_calls_hud_renderer() {
        // Arrange
        let mut sut = create_standard_test_game();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert_eq!(sut.hud_renderer.get_draw_calls().len(), 1);
    }

    #[test]
    fn start_level_sets_level_in_level_manager_and_playfield() {
        // Arrange
        let mut sut = create_standard_test_game();

        // Act
        sut.start_level(5);

        // Assert
        assert_eq!(sut.level_manager.get_current_level(), 5);
        assert_eq!(sut.get_playfield().get_gravity_timer().get_level(), 5);
    }
}
