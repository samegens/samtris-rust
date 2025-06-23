use crate::common::Dimensions;
use crate::common::Position;
use crate::constants::*;
use crate::game_logic::PlayfieldGrid;
use crate::graphics::PlayfieldView;
use crate::tetromino::TetrominoGenerator;
use crate::tetromino::TetrominoInstance;
use crate::tetromino::TetrominoType;

pub struct Playfield<T: TetrominoGenerator> {
    dimensions: Dimensions,
    grid: PlayfieldGrid,
    current_tetromino: Option<TetrominoInstance>,
    tetromino_generator: T,
}

impl<T: TetrominoGenerator> Playfield<T> {
    pub fn new(dimensions: Dimensions, tetromino_generator: T) -> Self {
        let grid = PlayfieldGrid::new(dimensions);
        Self {
            dimensions,
            grid,
            current_tetromino: None,
            tetromino_generator,
        }
    }

    pub fn get_current_tetromino(&self) -> Option<&TetrominoInstance> {
        self.current_tetromino.as_ref()
    }

    pub fn set_current_tetromino(&mut self, tetromino: Option<TetrominoInstance>) {
        self.current_tetromino = tetromino;
    }

    #[cfg(test)]
    pub fn get_tetromino_type_at(&self, position: Position) -> Option<TetrominoType> {
        self.grid.get(position).copied()
    }

    pub fn get_view(&self) -> PlayfieldView {
        PlayfieldView {
            dimensions: self.dimensions,
            grid: &self.grid,
            current_tetromino: self.current_tetromino.as_ref(),
        }
    }

    pub fn is_position_occupied(&self, position: Position) -> bool {
        self.grid.is_position_occupied(position)
    }

    pub fn spawn_tetromino(&mut self) -> bool {
        let position = Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y);
        let tetromino = self.tetromino_generator.generate(position);

        if !self.can_place_tetromino(&tetromino) {
            return false;
        }

        self.current_tetromino = Some(tetromino);
        true
    }

    /// Try to move the current tetromino. Returns true if the tetromino was moved successfully
    /// (there were no obstacles), false otherwise.
    pub fn try_move_current_tetromino<F>(&mut self, move_fn: F) -> bool
    where
        F: FnOnce(&mut TetrominoInstance),
    {
        if let Some(tetromino) = &self.current_tetromino {
            let mut moved_tetromino = tetromino.clone();
            move_fn(&mut moved_tetromino);

            if self.can_place_tetromino(&moved_tetromino) {
                self.set_current_tetromino(Some(moved_tetromino));
                return true;
            }
        }

        false
    }

    pub fn lock_tetromino(&mut self) {
        let tetromino = self.current_tetromino.as_ref().unwrap();
        let tetromino_type: TetrominoType = tetromino.get_type();
        let world_blocks: Vec<Position> = tetromino.get_world_blocks();

        for position in world_blocks {
            if self.dimensions.contains(position) {
                self.grid.set(position, Some(tetromino_type));
            }
        }

        self.current_tetromino = None;
    }

    pub fn can_place_tetromino(&self, tetromino: &TetrominoInstance) -> bool {
        let world_blocks: Vec<Position> = tetromino.get_world_blocks();
        for position in world_blocks {
            if !self.dimensions.contains(position) || self.is_position_occupied(position) {
                return false;
            }
        }

        true
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }

    // TODO: remove once lock_tetromino has moved to Playfield.
    pub fn get_full_lines(&self) -> Vec<u32> {
        self.grid.get_full_lines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{TETRIS_SPAWN_X, TETRIS_SPAWN_Y};
    use crate::test_helpers::*;
    use crate::tetromino::TetrominoDefinitions;
    use rstest::rstest;

    #[test]
    fn new_playfield_has_correct_dimensions() {
        // Act
        let sut = create_test_playfield();

        // Assert
        let expected_dimensions = Dimensions::new(PLAYFIELD_WIDTH, PLAYFIELD_HEIGHT);
        assert_eq!(sut.dimensions, expected_dimensions);
    }

    #[rstest]
    #[case(Position::new(-1, 0))]
    #[case(Position::new(10, 0))]
    #[case(Position::new(10, 20))]
    #[case(Position::new(0, 20))]
    fn get_tetromino_type_at_handles_out_of_bounds(#[case] position: Position) {
        // Arrange
        let sut = create_test_playfield();

        // Act
        let result: Option<TetrominoType> = sut.get_tetromino_type_at(position);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_tetromino_type_at_handles_unoccupied_position() {
        // Arrange
        let sut = create_test_playfield();
        let position = Position::new(TETRIS_SPAWN_X, TETRIS_SPAWN_Y);

        // Act
        let result: Option<TetrominoType> = sut.get_tetromino_type_at(position);

        // Assert
        assert!(result.is_none());
    }

    #[rstest]
    #[case(Position::new(1, 1))]
    #[case(Position::new(2, 1))]
    #[case(Position::new(1, 2))]
    #[case(Position::new(2, 2))]
    fn get_tetromino_type_at_handles_occupied_position(#[case] position: Position) {
        // Arrange
        let dimensions = Dimensions::new(4, 4);
        let mut sut = create_test_playfield_with_dimensions(dimensions);
        let spawn_position = Position::new(0, 0);
        let definitions = TetrominoDefinitions::new();
        let tetromino = TetrominoInstance::new(TetrominoType::O, spawn_position, &definitions);
        sut.set_current_tetromino(Some(tetromino));
        sut.lock_tetromino();

        // Act
        let result: Option<TetrominoType> = sut.get_tetromino_type_at(position);

        // Assert
        assert_eq!(result.unwrap(), TetrominoType::O);
    }

    #[rstest]
    #[case(Position::new(-1, 0))]
    #[case(Position::new(10, 0))]
    #[case(Position::new(10, 20))]
    #[case(Position::new(0, 20))]
    fn is_position_occupied_handles_out_of_bounds(#[case] position: Position) {
        // Arrange
        let sut = create_test_playfield();

        // Act
        let result = sut.is_position_occupied(position);

        // Assert
        assert!(!result)
    }

    #[test]
    fn place_tetromino_marks_all_block_positions_occupied() {
        // Arrange
        let mut sut = create_test_playfield();
        let definitions = TetrominoDefinitions::new();
        let tetromino = TetrominoInstance::new(TetrominoType::O, Position::new(5, 5), &definitions);
        sut.set_current_tetromino(Some(tetromino));

        // Act
        sut.lock_tetromino();

        // Assert
        assert!(!sut.is_position_occupied(Position::new(5, 5)));
        assert!(sut.is_position_occupied(Position::new(5 + 1, 5 + 1)));
        assert!(sut.is_position_occupied(Position::new(5 + 2, 5 + 1)));
        assert!(sut.is_position_occupied(Position::new(5 + 1, 5 + 2)));
        assert!(sut.is_position_occupied(Position::new(5 + 2, 5 + 2)));
        assert!(!sut.is_position_occupied(Position::new(5 + 3, 5 + 3)));
    }

    #[test]
    fn can_place_tetromino_returns_true_on_empty_playfield() {
        // Arrange
        let sut = create_test_playfield();
        let definitions = TetrominoDefinitions::new();
        let tetromino = TetrominoInstance::new(TetrominoType::O, Position::new(4, 4), &definitions);

        // Act
        let result = sut.can_place_tetromino(&tetromino);

        // Assert
        assert!(result);
    }

    #[rstest]
    #[case(Position::new(-4, 10), false)] // Too far left
    #[case(Position::new(-2, 10), false)] // Partially left
    #[case(Position::new(1, 10), true)] // Left edge (valid)
    #[case(Position::new(7, 10), true)] // Right edge (valid for O-piece)
    #[case(Position::new(8, 10), false)] // Partially right
    #[case(Position::new(9, 10), false)] // Too far right
    #[case(Position::new(4, 17), true)] // Bottom edge (valid for O-piece)
    #[case(Position::new(4, 18), false)] // Partially bottom
    #[case(Position::new(4, 19), false)] // Too far bottom
    fn can_place_tetromino_handles_bounds_checking(
        #[case] position: Position,
        #[case] expected_can_place: bool,
    ) {
        // Arrange
        let sut = create_test_playfield();
        let definitions = TetrominoDefinitions::new();
        let tetromino = TetrominoInstance::new(TetrominoType::O, position, &definitions);

        // Act
        let can_place = sut.can_place_tetromino(&tetromino);

        // Assert
        assert_eq!(can_place, expected_can_place);
    }

    #[rstest]
    #[case(Position::new(4, 4), Position::new(4, 4), false)] // Exact overlap
    #[case(Position::new(4, 4), Position::new(5, 4), false)] // Partial overlap (right)
    #[case(Position::new(4, 4), Position::new(3, 4), false)] // Partial overlap (left)
    #[case(Position::new(4, 4), Position::new(4, 5), false)] // Partial overlap (down)
    #[case(Position::new(4, 4), Position::new(4, 3), false)] // Partial overlap (up)
    #[case(Position::new(4, 4), Position::new(6, 4), true)] // Adjacent (right, no overlap)
    #[case(Position::new(4, 4), Position::new(2, 4), true)] // Adjacent (left, no overlap)
    #[case(Position::new(4, 4), Position::new(4, 6), true)] // Adjacent (down, no overlap)
    #[case(Position::new(4, 4), Position::new(4, 2), true)] // Adjacent (up, no overlap)
    fn can_place_tetromino_handles_overlapping_and_adjacent_pieces(
        #[case] first_position: Position,
        #[case] second_position: Position,
        #[case] expected_can_place: bool,
    ) {
        // Arrange
        let mut sut = create_test_playfield();
        let definitions = TetrominoDefinitions::new();
        let first_tetromino =
            TetrominoInstance::new(TetrominoType::O, first_position, &definitions);
        sut.set_current_tetromino(Some(first_tetromino));
        sut.lock_tetromino();
        let second_tetromino =
            TetrominoInstance::new(TetrominoType::O, second_position, &definitions);

        // Act
        let result = sut.can_place_tetromino(&second_tetromino);

        // Assert
        assert_eq!(result, expected_can_place);
    }

    #[test]
    fn clear_removes_all_placed_pieces() {
        // Arrange
        let mut sut = create_test_playfield();
        let definitions = TetrominoDefinitions::new();
        let tetromino = TetrominoInstance::new(TetrominoType::O, Position::new(2, 2), &definitions);
        sut.set_current_tetromino(Some(tetromino));
        sut.lock_tetromino();

        // Act
        sut.clear();

        // Assert
        for y in 0..sut.dimensions.height {
            for x in 0..sut.dimensions.width {
                assert!(!sut.is_position_occupied(Position::new(x as i32, y as i32)));
            }
        }
    }

    #[test]
    fn can_spawn_piece_in_new_playfield() {
        // Arrange
        let mut sut = create_test_playfield();

        // Act
        let result: bool = sut.spawn_tetromino();

        // Assert
        assert!(result);
        assert!(sut.get_current_tetromino().is_some());
    }

    #[test]
    fn cant_spawn_piece_on_top_of_occupied_blocks() {
        // Arrange
        let mut sut = create_test_playfield();
        sut.spawn_tetromino();
        sut.lock_tetromino();

        // Act
        let result: bool = sut.spawn_tetromino();

        // Assert
        assert!(!result);
    }

    #[rstest]
    #[case(|t: &mut TetrominoInstance| t.move_left(), -1, 0, 0)]
    #[case(|t: &mut TetrominoInstance| t.move_right(), 1, 0, 0)]
    #[case(|t: &mut TetrominoInstance| t.move_down(), 0, 1, 0)]
    #[case(|t: &mut TetrominoInstance| t.rotate_clockwise(), 0, 0, 1)]
    #[case(|t: &mut TetrominoInstance| t.rotate_counterclockwise(), 0, 0, 3)]
    fn can_move_tetromino_when_no_collision<F>(
        #[case] move_fn: F,
        #[case] expected_x_delta: i32,
        #[case] expected_y_delta: i32,
        #[case] expected_rotation_index: usize,
    ) where
        F: FnOnce(&mut TetrominoInstance),
    {
        // Arrange
        let mut sut = create_test_playfield_with_specific_type(TetrominoType::T);
        sut.spawn_tetromino();
        let initial_position = get_tetromino_start_position();

        // Act
        let result = sut.try_move_current_tetromino(move_fn);

        // Assert
        assert!(result);

        let current_tetromino = sut.get_current_tetromino().unwrap();
        let new_position = current_tetromino.get_position();
        let expected_position = Position::new(
            initial_position.x + expected_x_delta,
            initial_position.y + expected_y_delta,
        );
        assert_eq!(new_position, expected_position);

        let new_rotation_index: usize = current_tetromino.get_rotation_index().into();
        assert_eq!(new_rotation_index, expected_rotation_index);
    }

    #[rstest]
    #[case(|t: &mut TetrominoInstance| t.move_left(), TETRIS_SPAWN_X - 1, TETRIS_SPAWN_Y)]
    #[case(|t: &mut TetrominoInstance| t.move_right(), TETRIS_SPAWN_X + 1, TETRIS_SPAWN_Y)]
    #[case(|t: &mut TetrominoInstance| t.move_down(), TETRIS_SPAWN_X, TETRIS_SPAWN_Y + 4)]
    #[case(|t: &mut TetrominoInstance| t.rotate_clockwise(), TETRIS_SPAWN_X - 1, TETRIS_SPAWN_Y)]
    #[case(|t: &mut TetrominoInstance| t.rotate_counterclockwise(), TETRIS_SPAWN_X + 1, TETRIS_SPAWN_Y)]
    fn cant_move_tetromino_when_blocks_are_in_the_way<F>(
        #[case] move_fn: F,
        #[case] x_of_blocking_tetromino: i32,
        #[case] y_of_blocking_tetromino: i32,
    ) where
        F: FnOnce(&mut TetrominoInstance),
    {
        // Arrange
        let mut sut = create_test_playfield_with_specific_type(TetrominoType::I);

        // Place blocking tetromino
        let blocking_position = Position::new(x_of_blocking_tetromino, y_of_blocking_tetromino);
        let blocking_tetromino = create_tetromino_instance_at(TetrominoType::I, blocking_position);
        sut.set_current_tetromino(Some(blocking_tetromino));
        sut.lock_tetromino();

        // Spawn new tetromino
        sut.spawn_tetromino();
        let initial_position = sut.get_current_tetromino().unwrap().get_position();

        // Act
        let result = sut.try_move_current_tetromino(move_fn);

        // Assert
        assert!(!result);

        // Verify tetromino didn't move
        let final_position = sut.get_current_tetromino().unwrap().get_position();
        assert_eq!(final_position, initial_position);
    }
}
