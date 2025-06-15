#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

//TODO: remove allow dead_code when Color is used by application code
#[allow(dead_code)]
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

    #[test]
    fn color_new_creates_correct_color() {
        // Arrange & Act
        let color = Color::new(255, 128, 64);

        // Assert
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
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
