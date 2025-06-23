use crate::common::{Dimensions, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T: Clone> {
    dimensions: Dimensions,
    cells: Vec<Vec<Option<T>>>,
}

#[allow(dead_code)]
impl<T: Clone> Grid<T> {
    pub fn new(dimensions: Dimensions) -> Self {
        let cells = vec![vec![None; dimensions.width as usize]; dimensions.height as usize];
        Self { dimensions, cells }
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        if !self.dimensions.contains(position) {
            return None;
        }

        let x = position.x as usize;
        let y = position.y as usize;
        self.cells[y][x].as_ref()
    }

    pub fn set(&mut self, position: Position, value: Option<T>) {
        if !self.dimensions.contains(position) {
            return;
        }

        let x = position.x as usize;
        let y = position.y as usize;
        self.cells[y][x] = value;
    }

    pub fn is_position_occupied(&self, position: Position) -> bool {
        if !self.dimensions.contains(position) {
            return false;
        }

        let x = position.x as usize;
        let y = position.y as usize;
        self.cells[y][x].is_some()
    }

    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                *cell = None;
            }
        }
    }

    pub fn get_dimensions(&self) -> Dimensions {
        self.dimensions
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
        let sut: Grid<i32> = Grid::new(dimensions);

        // Assert
        assert_eq!(sut.get_dimensions(), dimensions);
        assert!(!sut.is_position_occupied(Position::new(0, 0)));
        assert!(!sut.is_position_occupied(Position::new(2, 1)));
    }

    #[test]
    fn set_and_get_stores_and_retrieves_value() {
        // Arrange
        let mut sut: Grid<i32> = Grid::new(Dimensions::new(3, 3));
        let position = Position::new(1, 2);

        // Act
        sut.set(position, Some(42));
        let result = sut.get(position);

        // Assert
        assert_eq!(result, Some(&42));
        assert!(sut.is_position_occupied(position));
    }

    #[test]
    fn set_with_none_clears_position() {
        // Arrange
        let mut sut: Grid<i32> = Grid::new(Dimensions::new(3, 3));
        let position = Position::new(1, 1);
        sut.set(position, Some(42));

        // Act
        sut.set(position, None);

        // Assert
        assert_eq!(sut.get(position), None);
        assert!(!sut.is_position_occupied(position));
    }

    #[test]
    fn get_returns_none_for_out_of_bounds() {
        // Arrange
        let sut: Grid<i32> = Grid::new(Dimensions::new(3, 3));

        // Act
        let result = sut.get(Position::new(-1, 0));

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn set_ignores_out_of_bounds() {
        // Arrange
        let mut sut: Grid<i32> = Grid::new(Dimensions::new(3, 3));

        // Act
        sut.set(Position::new(-1, 0), Some(42));

        // Assert (no panic, and grid unchanged)
        assert!(!sut.is_position_occupied(Position::new(0, 0)));
    }

    #[test]
    fn is_position_occupied_returns_false_for_out_of_bounds() {
        // Arrange
        let sut: Grid<i32> = Grid::new(Dimensions::new(3, 3));

        // Act
        let result = sut.is_position_occupied(Position::new(3, 0));

        // Assert
        assert!(!result);
    }

    #[test]
    fn clear_removes_all_values() {
        // Arrange
        let mut sut: Grid<i32> = Grid::new(Dimensions::new(2, 2));
        sut.set(Position::new(0, 0), Some(1));
        sut.set(Position::new(1, 1), Some(2));

        // Act
        sut.clear();

        // Assert
        assert!(!sut.is_position_occupied(Position::new(0, 0)));
        assert!(!sut.is_position_occupied(Position::new(1, 1)));
        assert_eq!(sut.get(Position::new(0, 0)), None);
        assert_eq!(sut.get(Position::new(1, 1)), None);
    }
}