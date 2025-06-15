use crate::constants::*;
use crate::game_input::GameInput;
use crate::playfield::Playfield;
use crate::position::Position;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimensions::Dimensions;
    use crate::game_input::GameInput;
    use crate::tetromino_type::TetrominoType;
    use rstest::rstest;

    fn create_test_game() -> Game {
        let playfield = create_test_playfield();
        Game::new(playfield)
    }

    fn create_test_playfield() -> Playfield {
        let dimensions = Dimensions::new(TETRIS_PLAYFIELD_WIDTH, TETRIS_PLAYFIELD_HEIGHT);
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
    #[case(GameInput::MoveLeft, -1, 0)]
    #[case(GameInput::MoveRight, 1, 0)]
    #[case(GameInput::MoveDown, 0, 1)]
    fn can_move_tetromino_when_no_collision(
        #[case] game_input: GameInput,
        #[case] x_delta: i32,
        #[case] y_delta: i32,
    ) {
        // Arrange
        let mut sut = create_test_game();
        sut.spawn_tetromino(TetrominoType::O);

        let initial_position = get_tetromino_start_position();

        // Act
        let result = sut.handle_input(game_input);

        // Assert
        assert!(result);
        let new_position = sut.get_current_tetromino().unwrap().get_position();
        let expected_position =
            Position::new(initial_position.x + x_delta, initial_position.y + y_delta);
        assert_eq!(new_position.x, expected_position.x);
    }

    #[rstest]
    #[case(GameInput::MoveLeft, TETRIS_SPAWN_X - 2, TETRIS_SPAWN_Y)]
    #[case(GameInput::MoveRight, TETRIS_SPAWN_X + 2, TETRIS_SPAWN_Y)]
    #[case(GameInput::MoveDown, TETRIS_SPAWN_X, TETRIS_SPAWN_Y + 2)]
    fn cant_move_tetromino_when_blocks_are_in_the_way(
        #[case] game_input: GameInput,
        #[case] x_of_blocking_tetromino: i32,
        #[case] y_of_blocking_tetromino: i32,
    ) {
        // Arrange
        let mut playfield = create_test_playfield();
        let position = Position::new(x_of_blocking_tetromino, y_of_blocking_tetromino);
        let tetromino = create_tetromino_instance_at(TetrominoType::O, position);
        playfield.place_tetromino(&tetromino);
        let mut sut = Game::new(playfield);
        sut.spawn_tetromino(TetrominoType::O);

        // Act
        let result = sut.handle_input(game_input);

        // Assert
        assert!(!result);
    }
}
