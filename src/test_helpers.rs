use crate::common::{Dimensions, Position};
use crate::constants::*;
use crate::game::Game;
use crate::graphics::MockPlayfieldRenderer;
use crate::playfield::Playfield;
use crate::tetromino::{
    FixedTetrominoGenerator, TetrominoDefinitions, TetrominoInstance, TetrominoType,
};

pub type TestGame = Game<MockPlayfieldRenderer, FixedTetrominoGenerator>;

pub fn create_test_game(tetromino_type: TetrominoType) -> TestGame {
    let playfield = create_test_playfield();
    let tetromino_generator = FixedTetrominoGenerator::new(tetromino_type);
    let renderer = MockPlayfieldRenderer::new();
    Game::new(playfield, tetromino_generator, renderer)
}

pub fn create_standard_test_game() -> TestGame {
    create_test_game(TetrominoType::O)
}

pub fn create_test_playfield() -> Playfield {
    let dimensions = Dimensions::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);
    Playfield::new(dimensions)
}

pub fn get_tetromino_start_position() -> Position {
    Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y)
}

pub fn lock_tetromino(game: &mut TestGame, tetromino_type: TetrominoType, position: Position) {
    let tetromino_instance = create_tetromino_instance_at(tetromino_type, position);
    game.get_playfield_mut()
        .set_current_tetromino(Some(tetromino_instance));
    game.get_playfield_mut().lock_tetromino();
}

pub fn create_tetromino_instance(tetromino_type: TetrominoType) -> TetrominoInstance {
    create_tetromino_instance_at(tetromino_type, get_tetromino_start_position())
}

pub fn create_tetromino_instance_at(
    tetromino_type: TetrominoType,
    position: Position,
) -> TetrominoInstance {
    let tetromino_definitions = TetrominoDefinitions::new();
    TetrominoInstance::new(tetromino_type, position, &tetromino_definitions)
}
