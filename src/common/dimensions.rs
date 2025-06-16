use crate::common::Position;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

impl Dimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn contains(&self, position: Position) -> bool {
        position.x >= 0
            && position.x < self.width as i32
            && position.y >= 0
            && position.y < self.height as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Position::new(0, 0), true)] // Inside
    #[case(Position::new(9, 19), true)] // Inside
    #[case(Position::new(-1, 10), false)] // Left of bounds
    #[case(Position::new(10, 10), false)] // Right of bounds
    #[case(Position::new(5, -1), false)] // Above bounds
    #[case(Position::new(5, 20), false)] // Below bounds
    #[case(Position::new(-1, -1), false)] // Top-left outside
    #[case(Position::new(10, -1), false)] // Top-right outside
    #[case(Position::new(-1, 20), false)] // Bottom-left outside
    #[case(Position::new(10, 20), false)] // Bottom-right outside
    fn contains_returns_correct_result_for_position(
        #[case] position: Position,
        #[case] expected: bool,
    ) {
        // Arrange
        let dimensions = Dimensions::new(10, 20);

        // Act
        let result = dimensions.contains(position);

        // Assert
        assert_eq!(result, expected);
    }
}
