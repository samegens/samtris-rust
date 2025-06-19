use crate::common::Position;
use crate::constants::*;
use crate::graphics::{Color, Display};
use crate::gui::GameInput;
use crate::playfield::Playfield;
use crate::tetromino_definitions::TetrominoDefinitions;
use crate::tetromino_instance::TetrominoInstance;
use crate::tetromino_type::TetrominoType;

pub struct Game {
    playfield: Playfield,
    tetromino_definitions: TetrominoDefinitions,
    current_tetromino: Option<TetrominoInstance>,
}

//TODO: remove allow dead_code when Game is used by application code
#[allow(dead_code)]
impl Game {
    pub fn new(playfield: Playfield) -> Self {
        Self {
            playfield,
            tetromino_definitions: TetrominoDefinitions::new(),
            current_tetromino: None,
        }
    }

    pub fn spawn_tetromino(&mut self, _tetromino_type: TetrominoType) -> bool {
        let position = Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y);
        let tetromino =
            TetrominoInstance::new(_tetromino_type, position, &self.tetromino_definitions);

        if !self.playfield.can_place_tetromino(&tetromino) {
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
            GameInput::MoveDown => self.try_move_piece(|tetromino| tetromino.move_down()),
            GameInput::RotateClockwise => {
                self.try_move_piece(|tetromino| tetromino.rotate_clockwise())
            }
            GameInput::RotateCounterclockwise => {
                self.try_move_piece(|tetromino| tetromino.rotate_counterclockwise())
            }
        }
    }

    fn try_move_piece<F>(&mut self, move_fn: F) -> bool
    where
        F: FnOnce(&mut TetrominoInstance),
    {
        if let Some(tetromino) = &self.current_tetromino {
            let mut new_tetromino = tetromino.clone();
            move_fn(&mut new_tetromino);

            if self.playfield.can_place_tetromino(&new_tetromino) {
                self.current_tetromino = Some(new_tetromino);
                return true;
            }
        }

        false
    }

    pub fn draw<D: Display>(&self, display: &mut D) -> Result<(), D::Error> {
        display.clear()?;
        self.draw_current_tetromino(display)?;
        Self::draw_playfield_border(display)?;
        display.present()?;

        Ok(())
    }

    fn draw_current_tetromino<D: Display>(&self, display: &mut D) -> Result<(), D::Error> {
        let x = PLAYFIELD_OFFSET_X as i32;
        let y = PLAYFIELD_OFFSET_Y as i32;
        let playfield_position = Position::new(x, y);

        if let Some(tetromino) = self.get_current_tetromino() {
            let blocks = tetromino.get_world_blocks();
            let tetromino_type = tetromino.get_type();

            for position in blocks {
                let window_position = playfield_position + position.scale(BLOCK_SIZE as i32);
                display.draw_block(window_position, tetromino_type)?;
            }
        }
        Ok(())
    }

    fn draw_playfield_border<D: Display>(display: &mut D) -> Result<(), D::Error> {
        let border_color = Color::WHITE;

        let mut x = PLAYFIELD_OFFSET_X - PLAYFIELD_BORDER_WIDTH;
        let mut y = PLAYFIELD_OFFSET_Y;
        let mut width = PLAYFIELD_BORDER_WIDTH;
        let mut height = PLAYFIELD_HEIGHT * BLOCK_SIZE;
        display.draw_rectangle(x, y, width, height, border_color)?;

        x = PLAYFIELD_OFFSET_X - PLAYFIELD_BORDER_WIDTH;
        y = PLAYFIELD_OFFSET_Y + PLAYFIELD_HEIGHT * BLOCK_SIZE;
        width = PLAYFIELD_BORDER_WIDTH + PLAYFIELD_WIDTH * BLOCK_SIZE + PLAYFIELD_BORDER_WIDTH;
        height = PLAYFIELD_BORDER_WIDTH;
        display.draw_rectangle(x, y, width, height, border_color)?;

        x = PLAYFIELD_OFFSET_X + PLAYFIELD_WIDTH * BLOCK_SIZE;
        y = PLAYFIELD_OFFSET_Y;
        width = PLAYFIELD_BORDER_WIDTH;
        height = PLAYFIELD_HEIGHT * BLOCK_SIZE;
        display.draw_rectangle(x, y, width, height, border_color)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Dimensions;
    use crate::graphics::mock_display::MockDisplay;
    use crate::gui::game_input::GameInput;
    use crate::tetromino_type::TetrominoType;
    use rstest::rstest;

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
        playfield.place_tetromino(&tetromino_instance);
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
        playfield.place_tetromino(&tetromino);
        let mut sut = Game::new(playfield);
        sut.spawn_tetromino(TetrominoType::I);

        // Act
        let result = sut.handle_input(game_input);

        // Assert
        assert!(!result);
    }

    #[test]
    fn draw_with_no_current_tetromino_only_draws_border() {
        // Arrange
        let playfield = Playfield::new(Dimensions::new(10, 20));
        let sut = Game::new(playfield); // No tetromino spawned
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.drawn_blocks.is_empty()); // No tetromino blocks
        assert!(!display.drawn_rectangles.is_empty()); // But border is drawn
    }

    #[test]
    fn draw_renders_current_tetromino_blocks() {
        // Arrange
        let playfield = Playfield::new(Dimensions::new(10, 20));
        let mut sut = Game::new(playfield);
        sut.spawn_tetromino(TetrominoType::O);
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(!display.drawn_blocks.is_empty());

        // Verify all drawn blocks are O-type
        for (_, tetromino_type) in &display.drawn_blocks {
            assert_eq!(*tetromino_type, TetrominoType::O);
        }
    }
}
