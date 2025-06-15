use crate::constants::{TETRIS_PLAYFIELD_HEIGHT, TETRIS_PLAYFIELD_WIDTH};
use crate::dimensions::Dimensions;
use crate::game::Game;
use crate::playfield::Playfield;
use crate::tetromino_type::TetrominoType;

mod constants;
mod dimensions;
mod game;
mod game_input;
mod playfield;
mod position;
mod rotation_index;
mod tetromino_definition;
mod tetromino_definitions;
mod tetromino_instance;
mod tetromino_type;

fn main() {
    let dimensions = Dimensions::new(TETRIS_PLAYFIELD_WIDTH, TETRIS_PLAYFIELD_HEIGHT);
    let playfield = Playfield::new(dimensions);
    let mut game = Game::new(playfield);
    game.spawn_tetromino(TetrominoType::O);
}
