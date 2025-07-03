// src/screens/high_scores_screen.rs
use crate::graphics::{Color, Display};
use crate::high_scores::HighScoreManager;
use crate::input::{InputEvent, Key};
use crate::screens::{Screen, ScreenResult};
use std::time::Duration;

pub struct HighScoresScreen {
    high_score_manager: HighScoreManager,
}

impl HighScoresScreen {
    pub fn new(high_score_manager: HighScoreManager) -> Self {
        Self { high_score_manager }
    }
}

impl Screen for HighScoresScreen {
    fn update(&mut self, _delta_time: Duration) {
        // No updates needed
    }

    fn draw(&mut self, display: &mut dyn Display) -> Result<(), String> {
        display.clear()?;

        const LEFT: u32 = 90;
        const TOP: u32 = 50;
        const LINE_HEIGHT: u32 = 25;
        display.draw_text("    SCORE   LEVEL  NAME", LEFT, TOP, Color::WHITE)?;

        let scores = self.high_score_manager.get_high_scores().get_scores();
        for (i, score) in scores.iter().enumerate() {
            let y = TOP + LINE_HEIGHT + (i as u32 * LINE_HEIGHT);
            let text = format!(
                "{:2}  {:06}  {:5}  {}",
                i + 1,
                score.score,
                score.level + 1,
                score.name
            );
            display.draw_text(&text, LEFT, y, Color::WHITE)?;
        }

        display.draw_text("Press ESC to return", 120, 350, Color::WHITE)?;

        display.present()?;
        Ok(())
    }

    fn handle_input(&mut self, input_events: &[InputEvent]) -> ScreenResult {
        for event in input_events {
            match event {
                InputEvent::Quit => return ScreenResult::Quit,
                InputEvent::KeyPressed(Key::Escape) => return ScreenResult::ReturnToMainMenu,
                _ => {}
            }
        }
        ScreenResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::MockDisplay;
    use crate::high_scores::{HighScore, HighScores, MockHighScoresRepository};

    #[test]
    fn new_creates_high_scores_screen() {
        // Arrange
        let repository = Box::new(MockHighScoresRepository::empty());
        let manager = HighScoreManager::new(repository);

        // Act
        let sut = HighScoresScreen::new(manager);

        // Assert
        assert_eq!(sut.high_score_manager.get_high_scores().len(), 0);
    }

    #[test]
    fn handle_input_escape_returns_to_main_menu() {
        // Arrange
        let repository = Box::new(MockHighScoresRepository::empty());
        let manager = HighScoreManager::new(repository);
        let mut sut = HighScoresScreen::new(manager);
        let input_events = vec![InputEvent::KeyPressed(Key::Escape)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::ReturnToMainMenu);
    }

    #[test]
    fn draw_displays_high_scores() {
        // Arrange
        let mut scores = HighScores::new();
        scores.add(HighScore::new("SAM".to_string(), 1000, 5));
        let repository = Box::new(MockHighScoresRepository::new(scores));
        let manager = HighScoreManager::new(repository);
        let mut sut = HighScoresScreen::new(manager);
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.cleared);
        assert!(display.presented);

        // Check that title and score were drawn
        let has_score = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text.contains("SAM"));
        assert!(has_score);
    }
}
