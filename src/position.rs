use std::ops::{Add, Sub};

/// 2D position in Tetris coordinate system (x: left→right, y: top→bottom).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn manhattan_distance(self, other: Self) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    pub fn translate(self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl From<Position> for (i32, i32) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn new_creates_position_with_correct_coordinates() {
        // Arrange
        let x = 5;
        let y = 10;

        // Act
        let position = Position::new(x, y);

        // Assert
        assert_eq!(position.x, x);
        assert_eq!(position.y, y);
    }

    #[test]
    fn origin_creates_position_at_zero_zero() {
        // Act
        let origin = Position::origin();

        // Assert
        assert_eq!(origin, Position::new(0, 0));
    }

    #[test]
    fn translate_moves_position_by_given_offset() {
        // Arrange
        let position = Position::new(5, 5);

        // Act
        let result = position.translate(2, -1);

        // Assert
        assert_eq!(result, Position::new(7, 4));
    }

    #[rstest]
    #[case(Position::new(1, 1), Position::new(4, 5), 7)]
    #[case(Position::new(0, 0), Position::new(0, 0), 0)]
    #[case(Position::new(-2, 3), Position::new(1, -1), 7)]
    fn manhattan_distance_calculates_correct_distance(
        #[case] pos1: Position,
        #[case] pos2: Position,
        #[case] expected: u32,
    ) {
        // Act
        let distance = pos1.manhattan_distance(pos2);

        // Assert
        assert_eq!(distance, expected);
    }

    #[test]
    fn add_returns_sum_of_two_positions() {
        // Arrange
        let pos1 = Position::new(2, 3);
        let pos2 = Position::new(4, 5);

        // Act
        let result = pos1 + pos2;

        // Assert
        assert_eq!(result, Position::new(6, 8));
    }

    #[rstest]
    #[case(Position::new(0, 0), Position::new(0, 0), Position::new(0, 0))]
    #[case(Position::new(1, 1), Position::new(4, 5), Position::new(5, 6))]
    fn translate_equals_add(
        #[case] pos1: Position,
        #[case] pos2: Position,
        #[case] expected: Position,
    ) {
        // Act
        let result = pos1 + pos2;

        // Assert
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(Position::new(5, 10), Position::new(5, 10), true)]
    #[case(Position::new(5, 10), Position::new(5, 11), false)]
    #[case(Position::new(0, 0), Position::new(0, 0), true)]
    #[case(Position::new(-3, 7), Position::new(-3, 7), true)]
    #[case(Position::new(-3, 7), Position::new(3, 7), false)]
    fn equality_comparison_works(
        #[case] pos1: Position,
        #[case] pos2: Position,
        #[case] should_be_equal: bool,
    ) {
        // Act
        let are_equal = pos1 == pos2;

        // Assert
        assert_eq!(are_equal, should_be_equal);
    }
}
