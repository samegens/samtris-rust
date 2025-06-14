use crate::constants::*;
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

    pub fn spawn_piece(&mut self, _tetromino_type: TetrominoType) -> bool {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimensions::Dimensions;
    use crate::tetromino_type::TetrominoType;

    #[test]
    fn new_game_has_no_current_tetromino() {
        // Arrange
        let dimensions = Dimensions::new(TETRIS_PLAYFIELD_WIDTH, TETRIS_PLAYFIELD_HEIGHT);
        let playfield = Playfield::new(dimensions);
        let sut = Game::new(playfield);

        // Act
        let result: Option<&TetrominoInstance> = sut.get_current_tetromino();

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn can_spawn_piece_in_new_game() {
        // Arrange
        let dimensions = Dimensions::new(TETRIS_PLAYFIELD_WIDTH, TETRIS_PLAYFIELD_HEIGHT);
        let playfield = Playfield::new(dimensions);
        let mut sut = Game::new(playfield);

        // Act
        let result: bool = sut.spawn_piece(TetrominoType::O);

        // Assert
        assert!(result);
        assert!(sut.get_current_tetromino().is_some());
    }

    #[test]
    fn cant_spawn_piece_on_top_of_occupied_blocks() {
        // Arrange
        let tetromino_definitions = TetrominoDefinitions::new();
        let dimensions = Dimensions::new(TETRIS_PLAYFIELD_WIDTH, TETRIS_PLAYFIELD_HEIGHT);
        let mut playfield = Playfield::new(dimensions);
        let position = Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y);
        let tetromino = TetrominoInstance::new(TetrominoType::O, position, &tetromino_definitions);
        playfield.place_tetromino(&tetromino);
        let mut sut = Game::new(playfield);

        // Act
        let result: bool = sut.spawn_piece(TetrominoType::O);

        // Assert
        assert!(!result);
    }
}
