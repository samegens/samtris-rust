#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuItem {
    StartGame,
    HighScores,
    Quit,
}

impl MenuItem {
    pub fn display_text(&self) -> &'static str {
        match self {
            MenuItem::StartGame => "START GAME",
            MenuItem::HighScores => "HIGH SCORES", 
            MenuItem::Quit => "QUIT",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(MenuItem::StartGame, "START GAME")]
    #[case(MenuItem::HighScores, "HIGH SCORES")]
    #[case(MenuItem::Quit, "QUIT")]
    fn display_text_returns_correct_string(
        #[case] menu_item: MenuItem,
        #[case] expected: &str,
    ) {
        // Act
        let result = menu_item.display_text();

        // Assert
        assert_eq!(result, expected);
    }
}
