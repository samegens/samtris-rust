use crate::common::{Dimensions, Position};
use crate::tetromino::TetrominoType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayfieldGrid {
    dimensions: Dimensions,
    cells: Vec<Vec<Option<TetrominoType>>>,
}

#[allow(dead_code)]
impl PlayfieldGrid {
    pub fn new(dimensions: Dimensions) -> Self {
        let cells = vec![vec![None; dimensions.width as usize]; dimensions.height as usize];
        Self { dimensions, cells }
    }

    pub fn get_dimensions(&self) -> Dimensions {
        self.dimensions
    }

    pub fn get(&self, position: Position) -> Option<&TetrominoType> {
        if !self.dimensions.contains(position) {
            return None;
        }

        let x = position.x as usize;
        let y = position.y as usize;
        self.cells[y][x].as_ref()
    }

    pub fn set(&mut self, position: Position, value: Option<TetrominoType>) {
        if !self.dimensions.contains(position) {
            return;
        }

        let x = position.x as usize;
        let y = position.y as usize;
        self.cells[y][x] = value;
    }

    pub fn is_position_occupied(&self, position: Position) -> bool {
        self.is_xy_occupied(position.x, position.y)
    }

    pub fn is_xy_occupied(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.dimensions.width as i32 || y < 0 || y >= self.dimensions.height as i32
        {
            return false;
        }

        self.cells[y as usize][x as usize].is_some()
    }

    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = None;
            }
        }
    }

    pub fn get_full_lines(&self) -> Vec<u32> {
        (0..self.dimensions.height)
            .filter(|&y| self.is_line_full(y))
            .collect()
    }

    fn is_line_full(&self, y: u32) -> bool {
        (0..self.dimensions.width).all(|x| self.is_xy_occupied(x as i32, y as i32))
    }

    pub fn remove_lines(&mut self, lines: &[u32]) {
        // Remove lines from bottom to top to avoid index shifting issues
        let mut sorted_lines = lines.to_vec();
        sorted_lines.sort();

        for &line_y in &sorted_lines {
            self.remove_line(line_y);
        }
    }

    fn remove_line(&mut self, line_y: u32) {
        if line_y >= self.dimensions.height {
            return;
        }

        self.cells.remove(line_y as usize);

        let empty_line = vec![None; self.dimensions.width as usize];
        self.cells.insert(0, empty_line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_grid_with_correct_dimensions() {
        // Arrange
        let dimensions = Dimensions::new(3, 2);

        // Act
        let sut: PlayfieldGrid = PlayfieldGrid::new(dimensions);

        // Assert
        assert_eq!(sut.get_dimensions(), dimensions);
        assert!(!sut.is_position_occupied(Position::new(0, 0)));
        assert!(!sut.is_position_occupied(Position::new(2, 1)));
    }

    #[test]
    fn set_and_get_stores_and_retrieves_value() {
        // Arrange
        let mut sut: PlayfieldGrid = PlayfieldGrid::new(Dimensions::new(3, 3));
        let position = Position::new(1, 2);

        // Act
        sut.set(position, Some(TetrominoType::I));
        let result = sut.get(position);

        // Assert
        assert_eq!(result, Some(&TetrominoType::I));
        assert!(sut.is_position_occupied(position));
    }

    #[test]
    fn set_with_none_clears_position() {
        // Arrange
        let mut sut: PlayfieldGrid = PlayfieldGrid::new(Dimensions::new(3, 3));
        let position = Position::new(1, 1);
        sut.set(position, Some(TetrominoType::I));

        // Act
        sut.set(position, None);

        // Assert
        assert_eq!(sut.get(position), None);
        assert!(!sut.is_position_occupied(position));
    }

    #[test]
    fn get_returns_none_for_out_of_bounds() {
        // Arrange
        let sut: PlayfieldGrid = PlayfieldGrid::new(Dimensions::new(3, 3));

        // Act
        let result = sut.get(Position::new(-1, 0));

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn set_ignores_out_of_bounds() {
        // Arrange
        let mut sut: PlayfieldGrid = PlayfieldGrid::new(Dimensions::new(3, 3));

        // Act
        sut.set(Position::new(-1, 0), Some(TetrominoType::I));

        // Assert (no panic, and grid unchanged)
        assert!(!sut.is_position_occupied(Position::new(0, 0)));
    }

    #[test]
    fn is_position_occupied_returns_false_for_out_of_bounds() {
        // Arrange
        let sut: PlayfieldGrid = PlayfieldGrid::new(Dimensions::new(3, 3));

        // Act
        let result = sut.is_position_occupied(Position::new(3, 0));

        // Assert
        assert!(!result);
    }

    #[test]
    fn clear_removes_all_values() {
        // Arrange
        let mut sut: PlayfieldGrid = PlayfieldGrid::new(Dimensions::new(2, 2));
        sut.set(Position::new(0, 0), Some(TetrominoType::I));
        sut.set(Position::new(1, 1), Some(TetrominoType::O));

        // Act
        sut.clear();

        // Assert
        assert!(!sut.is_position_occupied(Position::new(0, 0)));
        assert!(!sut.is_position_occupied(Position::new(1, 1)));
        assert_eq!(sut.get(Position::new(0, 0)), None);
        assert_eq!(sut.get(Position::new(1, 1)), None);
    }

    #[test]
    fn find_full_lines_returns_empty_for_empty_playfield() {
        // Arrange
        let dimensions = Dimensions::new(10, 20);
        let sut = PlayfieldGrid::new(dimensions);

        // Act
        let full_lines = sut.get_full_lines();

        // Assert
        assert!(full_lines.is_empty());
    }

    #[test]
    fn find_full_lines_detects_single_multiple_lines_in_small_playfield() {
        // Arrange
        let dimensions = Dimensions::new(1, 4);
        let mut sut = PlayfieldGrid::new(dimensions);
        for y in 0..4 {
            sut.set(Position::new(0, y), Some(TetrominoType::I));
        }

        // Act
        let full_lines = sut.get_full_lines();

        // Assert
        assert_eq!(full_lines, vec![0, 1, 2, 3]);
    }

    #[test]
    fn remove_lines_removes_full_line_and_drops_blocks_above() {
        // Arrange
        let mut sut = PlayfieldGrid::new(Dimensions::new(1, 5));

        // Setup:
        // Row 0: [O]
        // Row 1: [T]    <- full line to remove
        // Row 2: [S]
        // Row 3: [Z]    <- full line to remove
        // Row 4: [I]

        sut.set(Position::new(0, 0), Some(TetrominoType::O));
        sut.set(Position::new(0, 1), Some(TetrominoType::T));
        sut.set(Position::new(0, 2), Some(TetrominoType::S));
        sut.set(Position::new(0, 3), Some(TetrominoType::Z));
        sut.set(Position::new(0, 4), Some(TetrominoType::I));

        // Act
        sut.remove_lines(&[1, 3]);

        // Assert
        // Expected after removing line 1 and 3:
        // Row 0: [ ]  <- new empty line
        // Row 1: [ ]  <- new empty line
        // Row 2: [O]  <- original row 0
        // Row 3: [S]  <- original row 2
        // Row 4: [I]  <- original row 4

        assert_eq!(sut.get(Position::new(0, 0)), None);
        assert_eq!(sut.get(Position::new(0, 1)), None);
        assert_eq!(sut.get(Position::new(0, 2)), Some(&TetrominoType::O));
        assert_eq!(sut.get(Position::new(0, 3)), Some(&TetrominoType::S));
        assert_eq!(sut.get(Position::new(0, 4)), Some(&TetrominoType::I));
    }
}
