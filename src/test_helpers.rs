use std::sync::Arc;

use crate::common::{Dimensions, Position};
use crate::constants::*;
use crate::events::EventQueue;
use crate::game_logic::Game;
use crate::game_logic::Playfield;
use crate::graphics::{MockHudRenderer, MockPlayfieldRenderer};
use crate::high_scores::{HighScore, HighScoreManager, HighScores, MockHighScoresRepository};
use crate::screens::GameScreen;
use crate::tetromino::{
    FixedTetrominoGenerator, TetrominoDefinitions, TetrominoInstance, TetrominoType,
};

pub type TestGame = Game<MockPlayfieldRenderer, MockHudRenderer, FixedTetrominoGenerator>;

pub fn create_standard_test_game() -> TestGame {
    create_test_game(TetrominoType::O)
}

pub fn create_test_game(tetromino_type: TetrominoType) -> TestGame {
    let playfield = create_test_playfield_with_specific_type(tetromino_type);
    create_test_game_with_playfield(playfield)
}

pub fn create_test_game_with_playfield(playfield: Playfield<FixedTetrominoGenerator>) -> TestGame {
    Game::new(
        playfield,
        MockPlayfieldRenderer::new(),
        MockHudRenderer::new(),
        Arc::new(EventQueue::new()),
        create_test_high_score_manager(),
    )
}

pub fn create_test_playfield() -> Playfield<FixedTetrominoGenerator> {
    create_test_playfield_with_specific_type(TetrominoType::O)
}

pub fn create_test_playfield_with_event_queue(
    event_queue: Arc<EventQueue>,
) -> Playfield<FixedTetrominoGenerator> {
    let dimensions = Dimensions::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);
    Playfield::new(
        dimensions,
        FixedTetrominoGenerator::new(TetrominoType::O),
        event_queue,
    )
}

pub fn create_test_playfield_with_dimensions(
    dimensions: Dimensions,
) -> Playfield<FixedTetrominoGenerator> {
    let event_bus = Arc::new(EventQueue::new());
    Playfield::new(
        dimensions,
        FixedTetrominoGenerator::new(TetrominoType::O),
        event_bus,
    )
}

pub fn create_test_playfield_with_specific_type(
    tetromino_type: TetrominoType,
) -> Playfield<FixedTetrominoGenerator> {
    let dimensions = Dimensions::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);
    let event_bus = Arc::new(EventQueue::new());
    Playfield::new(
        dimensions,
        FixedTetrominoGenerator::new(tetromino_type),
        event_bus,
    )
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

pub fn create_tetromino_instance_at(
    tetromino_type: TetrominoType,
    position: Position,
) -> TetrominoInstance {
    let tetromino_definitions = TetrominoDefinitions::new();
    TetrominoInstance::new(tetromino_type, position, &tetromino_definitions)
}

pub fn get_tetromino_position_from_gamescreen(gamescreen: &GameScreen) -> Position {
    gamescreen
        .get_game()
        .get_playfield()
        .get_current_tetromino()
        .unwrap()
        .get_position()
}

pub fn create_test_high_score_manager() -> HighScoreManager {
    let high_scores_repository = MockHighScoresRepository::empty();
    HighScoreManager::new(Box::new(high_scores_repository))
}

pub fn create_test_high_score_manager_with_full_very_high_scores() -> HighScoreManager {
    let mut high_scores = HighScores::new();
    for i in 0..10 {
        high_scores.add(HighScore::new(
            format!("Player{i}"),
            100000 - (i * 1000),
            20,
        ));
    }
    let repository = MockHighScoresRepository::new(high_scores);
    HighScoreManager::new(Box::new(repository))
}
