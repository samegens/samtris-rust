// src/screens/enter_high_score_screen.rs
use crate::graphics::{Color, Display};
use crate::high_scores::{HighScore, HighScoreManager};
use crate::input::{InputEvent, Key};
use crate::screens::{Screen, ScreenResult};
use std::time::Duration;

pub struct EnterHighScoreScreen {
    high_score_manager: HighScoreManager,
    player_name: String,
    score: u32,
    level: u32,
}

impl EnterHighScoreScreen {
    pub fn new(high_score_manager: HighScoreManager, score: u32, level: u32) -> Self {
        Self {
            high_score_manager,
            player_name: String::new(),
            score,
            level,
        }
    }

    fn save_high_score(&mut self) -> Result<(), String> {
        let name = if self.player_name.is_empty() {
            "PLAYER".to_string()
        } else {
            self.player_name.clone()
        };

        let high_score = HighScore::new(name, self.score, self.level);
        self.high_score_manager.add_high_score(high_score)?;
        Ok(())
    }
}

impl Screen for EnterHighScoreScreen {
    fn update(&mut self, _delta_time: Duration) {
        // No updates needed yet
    }

    fn draw(&mut self, display: &mut dyn Display) -> Result<(), String> {
        display.clear()?;

        display.draw_text("NEW HIGH SCORE!", 90, 50, Color::WHITE)?;
        display.draw_text(&format!("Score: {:06}", self.score), 90, 80, Color::WHITE)?;
        display.draw_text(&format!("Level: {}", self.level + 1), 90, 110, Color::WHITE)?;

        display.draw_text("Enter your name:", 90, 160, Color::WHITE)?;

        if !self.player_name.is_empty() {
            display.draw_text(&self.player_name, 90, 190, Color::WHITE)?;
        }

        display.draw_text("Press ENTER to save", 90, 240, Color::WHITE)?;

        display.present()?;
        Ok(())
    }

    fn handle_input(&mut self, input_events: &[InputEvent]) -> ScreenResult {
        for event in input_events {
            match event {
                InputEvent::Quit => return ScreenResult::Quit,
                InputEvent::KeyPressed(Key::Escape) => return ScreenResult::ShowHighScores,
                InputEvent::KeyPressed(Key::Enter) => {
                    if let Err(e) = self.save_high_score() {
                        eprintln!("Failed to save high score: {}", e);
                    }
                    return ScreenResult::ShowHighScores;
                }
                InputEvent::KeyPressed(_) => {
                    // Character input will be added next
                }
                InputEvent::KeyReleased(_) => {}
            }
        }
        ScreenResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::MockDisplay;
    use crate::high_scores::MockHighScoresRepository;

    #[test]
    fn new_creates_enter_high_score_screen() {
        // Act
        let sut = create_test_screen();

        // Assert
        assert_eq!(sut.score, 1500);
        assert_eq!(sut.level, 3);
        assert!(sut.player_name.is_empty());
    }

    #[test]
    fn handle_input_escape_returns_to_high_scores() {
        // Arrange
        let mut sut = create_test_screen();
        let input_events = vec![InputEvent::KeyPressed(Key::Escape)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::ShowHighScores);
    }

    #[test]
    fn handle_input_enter_saves_score_and_shows_high_scores() {
        // Arrange
        let mut sut = create_test_screen();
        sut.player_name = "SAM".to_string();
        let input_events = vec![InputEvent::KeyPressed(Key::Enter)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::ShowHighScores);
        assert_eq!(sut.high_score_manager.get_high_scores().len(), 1);
    }

    #[test]
    fn draw_displays_score_info() {
        // Arrange
        let mut sut = create_test_screen();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.cleared);
        assert!(display.presented);

        let has_high_score_text = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text.contains("NEW HIGH SCORE"));
        assert!(has_high_score_text);
    }

    #[test]
    fn draw_does_not_try_to_draw_text_for_empty_name() {
        // Arrange
        let mut sut = create_test_screen();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        let has_drawn_empty_text = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text.is_empty());
        assert!(!has_drawn_empty_text);
    }

    fn create_test_screen() -> EnterHighScoreScreen {
        let repository = Box::new(MockHighScoresRepository::empty());
        let manager = HighScoreManager::new(repository);
        EnterHighScoreScreen::new(manager, 1500, 3)
    }
}
