use crate::common::Dimensions;
use crate::common::Position;
use crate::tetromino::TetrominoInstance;
use crate::tetromino::TetrominoType;

pub struct Playfield {
    dimensions: Dimensions,
    grid: Vec<Vec<Option<TetrominoType>>>,
    current_tetromino: Option<TetrominoInstance>,
}

impl Playfield {
    pub fn new(dimensions: Dimensions) -> Self {
        let grid: Vec<Vec<Option<TetrominoType>>> =
            vec![vec![None; dimensions.width as usize]; dimensions.height as usize];
        Self {
            dimensions,
            grid,
            current_tetromino: None,
        }
    }

    pub fn get_dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn get_current_tetromino(&self) -> Option<&TetrominoInstance> {
        self.current_tetromino.as_ref()
    }

    pub fn set_current_tetromino(&mut self, tetromino: Option<TetrominoInstance>) {
        self.current_tetromino = tetromino;
    }

    pub fn get_tetromino_type_at(&self, position: Position) -> Option<TetrominoType> {
        if !self.dimensions.contains(position) {
            return None;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        self.grid[y][x]
    }

    pub fn is_position_occupied(&self, position: Position) -> bool {
        if !self.dimensions.contains(position) {
            return false;
        }

        let x = position.x as u32;
        let y = position.y as u32;

        self.is_xy_occupied(x, y)
    }

    fn is_xy_occupied(&self, x: u32, y: u32) -> bool {
        self.grid[y as usize][x as usize].is_some()
    }

    pub fn spawn_tetromino(&mut self, tetromino: TetrominoInstance) -> bool {
        if !self.can_place_tetromino(&tetromino) {
            return false;
        }

        self.current_tetromino = Some(tetromino);
        true
    }

    pub fn lock_tetromino(&mut self) {
        let tetromino = self.current_tetromino.as_ref().unwrap();
        let tetromino_type: TetrominoType = tetromino.get_type();
        let world_blocks: Vec<Position> = tetromino.get_world_blocks();

        for position in world_blocks {
            if self.dimensions.contains(position) {
                let x = position.x as usize;
                let y = position.y as usize;

                self.grid[y][x] = Some(tetromino_type);
            }
        }
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

    pub fn get_full_lines(&self) -> Vec<u32> {
        (0..self.dimensions.height)
            .filter(|&y| self.is_line_full(y))
            .collect()
    }

    fn is_line_full(&self, y: u32) -> bool {
        (0..self.dimensions.width).all(|x| self.is_xy_occupied(x, y))
    }

    pub fn clear(&mut self) {
        self.grid =
            vec![vec![None; self.dimensions.width as usize]; self.dimensions.height as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{TETRIS_SPAWN_X, TETRIS_SPAWN_Y};
    use crate::tetromino::TetrominoDefinitions;
    use rstest::rstest;

    #[test]
    fn new_playfield_has_correct_dimensions() {
        // Arrange
        let dimensions = Dimensions::new(7, 11);

        // Act
        let sut = Playfield::new(dimensions);

        // Assert
        assert_eq!(sut.get_dimensions(), dimensions);
    }

    #[rstest]
    #[case(Position::new(-1, 0))]
    #[case(Position::new(10, 0))]
    #[case(Position::new(10, 20))]
    #[case(Position::new(0, 20))]
    fn get_tetromino_type_at_handles_out_of_bounds(#[case] position: Position) {
        // Arrange
        let dimensions = Dimensions::new(10, 20);
        let sut = Playfield::new(dimensions);

        // Act
        let result: Option<TetrominoType> = sut.get_tetromino_type_at(position);

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn get_tetromino_type_at_handles_unoccupied_position() {
        // Arrange
        let dimensions = Dimensions::new(10, 20);
        let sut = Playfield::new(dimensions);
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
        let mut sut = Playfield::new(dimensions);
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
        let dimensions = Dimensions::new(10, 20);
        let sut = Playfield::new(dimensions);

        // Act
        let result = sut.is_position_occupied(position);

        // Assert
        assert!(!result)
    }

    #[test]
    fn place_tetromino_marks_all_block_positions_occupied() {
        // Arrange
        let dimensions = Dimensions::new(10, 20);
        let mut sut = Playfield::new(dimensions);
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
        let dimensions = Dimensions::new(10, 20);
        let sut = Playfield::new(dimensions);
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
        let dimensions = Dimensions::new(10, 20);
        let sut = Playfield::new(dimensions);
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
        let dimensions = Dimensions::new(10, 20);
        let mut sut = Playfield::new(dimensions);
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
    fn find_full_lines_returns_empty_for_empty_playfield() {
        // Arrange
        let dimensions = Dimensions::new(10, 20);
        let playfield = Playfield::new(dimensions);

        // Act
        let full_lines = playfield.get_full_lines();

        // Assert
        assert!(full_lines.is_empty());
    }

    #[test]
    fn find_full_lines_detects_single_full_line_in_small_playfield() {
        // Arrange
        let dimensions = Dimensions::new(4, 1);
        let mut playfield = Playfield::new(dimensions);
        let definitions = TetrominoDefinitions::new();
        let mut tetromino =
            TetrominoInstance::new(TetrominoType::I, Position::new(0, -1), &definitions);
        tetromino.rotate_clockwise();
        // Place I-piece horizontally to fill the single row.
        playfield.set_current_tetromino(Some(tetromino));
        playfield.lock_tetromino();

        // Act
        let full_lines = playfield.get_full_lines();

        // Assert
        assert_eq!(full_lines, vec![0]);
    }

    #[test]
    fn find_full_lines_detects_single_multiple_lines_in_small_playfield() {
        // Arrange
        let dimensions = Dimensions::new(1, 4);
        let mut playfield = Playfield::new(dimensions);
        let definitions = TetrominoDefinitions::new();
        let tetromino =
            TetrominoInstance::new(TetrominoType::I, Position::new(-1, 0), &definitions);
        // Place I-piece vertically to fill 4 rows.
        playfield.set_current_tetromino(Some(tetromino));
        playfield.lock_tetromino();

        // Act
        let full_lines = playfield.get_full_lines();

        // Assert
        assert_eq!(full_lines, vec![0, 1, 2, 3]);
    }

    #[test]
    fn clear_removes_all_placed_pieces() {
        // Arrange
        let dimensions = Dimensions::new(10, 20);
        let mut sut = Playfield::new(dimensions);
        let definitions = TetrominoDefinitions::new();

        // Place several tetrominos on the playfield
        let tetromino = TetrominoInstance::new(TetrominoType::O, Position::new(2, 2), &definitions);
        sut.set_current_tetromino(Some(tetromino));
        sut.lock_tetromino();

        // Act
        sut.clear();

        // Assert
        for y in 0..dimensions.height {
            for x in 0..dimensions.width {
                assert!(!sut.is_position_occupied(Position::new(x as i32, y as i32)));
            }
        }
    }
}
