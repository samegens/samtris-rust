#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    // Common colors
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const GRAY: Color = Color::new(128, 128, 128);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const PURPLE: Color = Color::new(128, 0, 128);
    pub const ORANGE: Color = Color::new(255, 165, 0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn color_new_creates_correct_color() {
        // Arrange & Act
        let color = Color::new(255, 128, 64);

        // Assert
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[rstest]
    #[case(Color::BLACK, 0, 0, 0)]
    #[case(Color::WHITE, 255, 255, 255)]
    #[case(Color::RED, 255, 0, 0)]
    #[case(Color::GREEN, 0, 255, 0)]
    #[case(Color::BLUE, 0, 0, 255)]
    #[case(Color::CYAN, 0, 255, 255)]
    #[case(Color::YELLOW, 255, 255, 0)]
    #[case(Color::PURPLE, 128, 0, 128)]
    #[case(Color::ORANGE, 255, 165, 0)]
    #[case(Color::GRAY, 128, 128, 128)]
    fn color_constants_have_correct_values(
        #[case] color: Color,
        #[case] expected_r: u8,
        #[case] expected_g: u8,
        #[case] expected_b: u8,
    ) {
        // Assert
        assert_eq!(color.r, expected_r);
        assert_eq!(color.g, expected_g);
        assert_eq!(color.b, expected_b);
    }
    #[test]
    fn colors_are_copyable() {
        // Arrange
        let color1 = Color::RED;

        // Act
        let color2 = color1; // Should copy, not move

        // Assert
        assert_eq!(color1, color2); // color1 should still be usable
    }
}
