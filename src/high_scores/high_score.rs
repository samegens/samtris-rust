#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighScore {
    pub name: String,
    pub score: u32,
    pub level: u32,
}

impl HighScore {
    pub fn new(name: String, score: u32, level: u32) -> Self {
        Self { name, score, level }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_high_score_with_correct_values() {
        // Act
        let sut = HighScore::new("SAM".to_string(), 1000, 5);

        // Assert
        assert_eq!(sut.name, "SAM");
        assert_eq!(sut.score, 1000);
        assert_eq!(sut.level, 5);
    }
}
