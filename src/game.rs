use crate::common::Position;
use crate::constants::*;
use crate::graphics::{Display, PlayfieldRenderer};
use crate::gravity_timer::GravityTimer;
use crate::gui::GameInput;
use crate::playfield::Playfield;
use crate::tetromino_definitions::TetrominoDefinitions;
use crate::tetromino_instance::TetrominoInstance;
use crate::tetromino_type::TetrominoType;
use std::time::Duration;

pub struct Game {
    playfield: Playfield,
    tetromino_definitions: TetrominoDefinitions,
    current_tetromino: Option<TetrominoInstance>,
    gravity_timer: GravityTimer,
    renderer: PlayfieldRenderer,
}

//TODO: remove allow dead_code when Game is used by application code
#[allow(dead_code)]
impl Game {
    pub fn new(playfield: Playfield) -> Self {
        let level: usize = 0;
        let gravity_timer = GravityTimer::new(level);
        Self {
            playfield,
            tetromino_definitions: TetrominoDefinitions::new(),
            current_tetromino: None,
            gravity_timer,
            renderer: PlayfieldRenderer::new(),
        }
    }

    pub fn get_playfield(&self) -> &Playfield {
        &self.playfield
    }

    pub fn spawn_tetromino(&mut self, _tetromino_type: TetrominoType) -> bool {
        let position = Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y);
        let tetromino =
            TetrominoInstance::new(_tetromino_type, position, &self.tetromino_definitions);

        if !self.playfield.can_place_tetromino(&tetromino) {
            // TODO: this means end of game.
            return false;
        }

        self.current_tetromino = Some(tetromino);
        true
    }

    pub fn get_current_tetromino(&self) -> Option<&TetrominoInstance> {
        self.current_tetromino.as_ref()
    }

    pub fn handle_input(&mut self, input: GameInput) -> bool {
        match input {
            GameInput::MoveLeft => self.try_move_piece(|tetromino| tetromino.move_left()),
            GameInput::MoveRight => self.try_move_piece(|tetromino| tetromino.move_right()),
            GameInput::MoveDown => {
                let has_moved: bool = self.try_move_piece(|tetromino| tetromino.move_down());
                if has_moved {
                    self.gravity_timer.reset();
                } else {
                    self.lock_current_tetromino();
                }
                has_moved
            }
            GameInput::RotateClockwise => {
                self.try_move_piece(|tetromino| tetromino.rotate_clockwise())
            }
            GameInput::RotateCounterclockwise => {
                self.try_move_piece(|tetromino| tetromino.rotate_counterclockwise())
            }
        }
    }

    /// Try to move the current tetromino. Returns true if the tetromino was moved successfully
    /// (there were no obstacles), false otherwise.
    fn try_move_piece<F>(&mut self, move_fn: F) -> bool
    where
        F: FnOnce(&mut TetrominoInstance),
    {
        if let Some(tetromino) = &self.current_tetromino {
            let mut moved_tetromino = tetromino.clone();
            move_fn(&mut moved_tetromino);

            if self.playfield.can_place_tetromino(&moved_tetromino) {
                self.current_tetromino = Some(moved_tetromino);
                return true;
            }
        }

        false
    }

    pub fn draw<D: Display>(&self, display: &mut D) -> Result<(), D::Error> {
        display.clear()?;
        self.renderer
            .draw(&self.playfield, self.current_tetromino.as_ref(), display)?;
        display.present()?;
        Ok(())
    }

    pub fn update(&mut self, delta_time: Duration) {
        if self.current_tetromino.is_some() && self.gravity_timer.update(delta_time) {
            self.apply_gravity();
        }
    }

    fn apply_gravity(&mut self) {
        let moved = self.try_move_piece(|tetromino| tetromino.move_down());

        if !moved {
            self.lock_current_tetromino();
        }
    }

    /// Locks the current tetromino in its current position and spawns a new tetromino in the
    /// start position.
    fn lock_current_tetromino(&mut self) {
        self.playfield
            .lock_tetromino(self.current_tetromino.as_ref().unwrap());
        self.spawn_tetromino(TetrominoType::O);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Dimensions;
    use crate::graphics::MockDisplay;
    use crate::gui::game_input::GameInput;
    use crate::tetromino_type::TetrominoType;
    use rstest::rstest;
    use std::time::Duration;

    fn create_test_game() -> Game {
        let playfield = create_test_playfield();
        Game::new(playfield)
    }

    fn create_test_playfield() -> Playfield {
        let dimensions = Dimensions::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);
        Playfield::new(dimensions)
    }

    fn get_tetromino_start_position() -> Position {
        Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y)
    }

    fn create_tetromino_instance(tetromino_type: TetrominoType) -> TetrominoInstance {
        create_tetromino_instance_at(tetromino_type, get_tetromino_start_position())
    }

    fn create_tetromino_instance_at(
        tetromino_type: TetrominoType,
        position: Position,
    ) -> TetrominoInstance {
        let tetromino_definitions = TetrominoDefinitions::new();
        TetrominoInstance::new(tetromino_type, position, &tetromino_definitions)
    }

    #[test]
    fn new_game_has_no_current_tetromino() {
        // Arrange
        let sut = create_test_game();

        // Act
        let result: Option<&TetrominoInstance> = sut.get_current_tetromino();

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn can_spawn_piece_in_new_game() {
        // Arrange
        let mut sut = create_test_game();

        // Act
        let result: bool = sut.spawn_tetromino(TetrominoType::O);

        // Assert
        assert!(result);
        assert!(sut.get_current_tetromino().is_some());
    }

    #[test]
    fn cant_spawn_piece_on_top_of_occupied_blocks() {
        // Arrange
        let mut playfield = create_test_playfield();
        let tetromino_instance = create_tetromino_instance(TetrominoType::O);
        playfield.lock_tetromino(&tetromino_instance);
        let mut sut = Game::new(playfield);

        // Act
        let result: bool = sut.spawn_tetromino(TetrominoType::O);

        // Assert
        assert!(!result);
    }

    #[rstest]
    #[case(GameInput::MoveLeft, -1, 0, 0)]
    #[case(GameInput::MoveRight, 1, 0, 0)]
    #[case(GameInput::MoveDown, 0, 1, 0)]
    #[case(GameInput::RotateClockwise, 0, 0, 1)]
    #[case(GameInput::RotateCounterclockwise, 0, 0, 3)]
    fn can_move_tetromino_when_no_collision(
        #[case] game_input: GameInput,
        #[case] expected_x_delta: i32,
        #[case] expected_y_delta: i32,
        #[case] expected_rotation_index: usize,
    ) {
        // Arrange
        let mut sut = create_test_game();
        sut.spawn_tetromino(TetrominoType::T);

        let initial_position = get_tetromino_start_position();

        // Act
        let result = sut.handle_input(game_input);

        // Assert
        assert!(result);

        let new_position = sut.get_current_tetromino().unwrap().get_position();
        let expected_position = Position::new(
            initial_position.x + expected_x_delta,
            initial_position.y + expected_y_delta,
        );
        assert_eq!(new_position.x, expected_position.x);

        let new_rotation_index: usize = sut
            .get_current_tetromino()
            .unwrap()
            .get_rotation_index()
            .into();
        assert_eq!(new_rotation_index, expected_rotation_index);
    }

    #[rstest]
    #[case(GameInput::MoveLeft, TETRIS_SPAWN_X - 1, TETRIS_SPAWN_Y)]
    #[case(GameInput::MoveRight, TETRIS_SPAWN_X + 1, TETRIS_SPAWN_Y)]
    #[case(GameInput::MoveDown, TETRIS_SPAWN_X, TETRIS_SPAWN_Y + 4)]
    #[case(GameInput::RotateClockwise, TETRIS_SPAWN_X - 1, TETRIS_SPAWN_Y)]
    #[case(GameInput::RotateCounterclockwise, TETRIS_SPAWN_X + 1, TETRIS_SPAWN_Y)]
    fn cant_move_tetromino_when_blocks_are_in_the_way(
        #[case] game_input: GameInput,
        #[case] x_of_blocking_tetromino: i32,
        #[case] y_of_blocking_tetromino: i32,
    ) {
        // Arrange
        let mut playfield = create_test_playfield();
        let position = Position::new(x_of_blocking_tetromino, y_of_blocking_tetromino);
        let tetromino = create_tetromino_instance_at(TetrominoType::I, position);
        playfield.lock_tetromino(&tetromino);
        let mut sut = Game::new(playfield);
        sut.spawn_tetromino(TetrominoType::I);

        // Act
        let result = sut.handle_input(game_input);

        // Assert
        assert!(!result);
    }

    #[test]
    fn tetromino_falls_automatically_when_gravity_timer_expires() {
        // Arrange
        let mut sut = create_test_game();
        sut.spawn_tetromino(TetrominoType::I);
        let initial_position = sut.get_current_tetromino().unwrap().get_position();

        // Act
        sut.update(Duration::from_millis(1000));

        // Assert
        let new_position = sut.get_current_tetromino().unwrap().get_position();
        assert_eq!(new_position.y, initial_position.y + 1);
    }

    #[test]
    fn tetromino_does_not_fall_automatically_before_gravity_timer_expires() {
        // Arrange
        let mut sut = create_test_game();
        sut.spawn_tetromino(TetrominoType::I);
        let initial_position = sut.get_current_tetromino().unwrap().get_position();

        // Act
        sut.update(Duration::from_millis(500));

        // Assert
        let new_position = sut.get_current_tetromino().unwrap().get_position();
        assert_eq!(new_position.y, initial_position.y);
    }

    #[test]
    fn handle_input_move_down_returns_false_when_tetromino_cannot_move() {
        // Arrange
        let mut playfield = create_test_playfield();
        // Place an O-tetromino 4 lines below the spawn line so the locked O will fit and a new O.
        let position = Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y + 4);
        let blocking_tetromino = create_tetromino_instance_at(TetrominoType::O, position);
        playfield.lock_tetromino(&blocking_tetromino);

        let mut sut = Game::new(playfield);
        sut.spawn_tetromino(TetrominoType::O);

        sut.handle_input(GameInput::MoveDown);
        sut.handle_input(GameInput::MoveDown);

        // Act
        let result = sut.handle_input(GameInput::MoveDown);

        // Assert
        assert!(!result);
        assert_eq!(
            sut.get_current_tetromino().unwrap().get_position(),
            Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y)
        );

        let locked_position = Position::new(TETRIS_SPAWN_X + 1, TETRIS_SPAWN_Y + 3);
        assert!(sut.get_playfield().is_position_occupied(locked_position));
    }

    #[test]
    fn tetromino_locks_when_gravity_cannot_move_it_down() {
        // Arrange
        let mut playfield = Playfield::new(Dimensions::new(PLAYFIELD_WIDTH, 5)); // Small playfield
        let definitions = TetrominoDefinitions::new();

        // Place an O tetromino directly below spawn position to block movement
        let blocking_tetromino = TetrominoInstance::new(
            TetrominoType::O,
            Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y + 2),
            &definitions,
        );
        playfield.lock_tetromino(&blocking_tetromino);

        let mut sut = Game::new(playfield);
        sut.spawn_tetromino(TetrominoType::O);

        // Act
        sut.update(Duration::from_millis(1000)); // Trigger gravity

        // Assert
        assert!(sut.get_current_tetromino().is_some());
        assert_eq!(
            sut.get_current_tetromino().unwrap().get_position(),
            Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y)
        );
        assert!(sut
            .get_playfield()
            .is_position_occupied(Position::new(TETRIS_SPAWN_X + 1, TETRIS_SPAWN_Y + 1)));
    }

    #[test]
    fn draw_coordinates_display_lifecycle() {
        // Arrange
        let mut sut = create_test_game();
        sut.spawn_tetromino(TetrominoType::O);
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.cleared); // Game called clear
        assert!(display.presented); // Game called present
                                    // The renderer was called (evidenced by drawing operations between clear and present)
        assert!(!display.drawn_rectangles.is_empty()); // Border drawn by renderer
        assert!(!display.drawn_blocks.is_empty()); // Tetromino drawn by renderer
    }
}
