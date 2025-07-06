use crate::constants::*;
use crate::graphics::{Color, Display};
use crate::high_scores::{HighScoreManager, HighScoresScreenBackground};
use crate::input::{InputEvent, Key};
use crate::screens::{Screen, ScreenResult};
use std::time::Duration;

pub struct HighScoresScreen {
    high_score_manager: HighScoreManager,
    background: HighScoresScreenBackground,
}

impl HighScoresScreen {
    pub fn new(high_score_manager: HighScoreManager) -> Self {
        Self {
            high_score_manager,
            background: HighScoresScreenBackground::new(),
        }
    }
}

impl Screen for HighScoresScreen {
    fn update(&mut self, _delta_time: Duration) {
        // No updates needed
    }

    fn draw(&mut self, display: &mut dyn Display) -> Result<(), String> {
        display.clear()?;

        self.background.draw(display)?;

        display.draw_text(
            "    SCORE   LEVEL  NAME",
            HIGH_SCORES_X,
            HIGH_SCORES_Y,
            Color::WHITE,
        )?;

        let scores = self.high_score_manager.get_high_scores().get_scores();
        for (i, score) in scores.iter().enumerate() {
            let y = HIGH_SCORES_Y + HIGH_SCORES_LINE_HEIGHT + (i as u32 * HIGH_SCORES_LINE_HEIGHT);
            let text = format!(
                "{:2}  {:06}  {:5}  {}",
                i + 1,
                score.game_result.score,
                score.game_result.level + 1,
                score.name
            );
            display.draw_text(&text, HIGH_SCORES_X, y, Color::WHITE)?;
        }

        display.draw_text("Press ESC to return", 150, 320, Color::WHITE)?;

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
    use crate::game_logic::GameResult;
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
        scores.add(HighScore::new(
            "SAM".to_string(),
            GameResult {
                score: 1000,
                level: 5,
            },
        ));
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

    #[test]
    fn update_does_not_change_state() {
        // Arrange
        let repository = Box::new(MockHighScoresRepository::empty());
        let mut manager = HighScoreManager::new(repository);
        for i in 0..7 {
            let name = format!("PLAYER{i}");
            let score = 1000 + i * 100;
            let level = i;
            let high_score = HighScore::new(name, GameResult { score, level });
            manager.add_high_score(high_score).unwrap();
        }
        let mut sut = HighScoresScreen::new(manager);
        let initial_scores_len = sut.high_score_manager.get_high_scores().len();

        // Act
        sut.update(Duration::from_millis(100));

        // Assert
        assert_eq!(
            sut.high_score_manager.get_high_scores().len(),
            initial_scores_len
        );
    }

    #[test]
    fn handle_input_quit_returns_quit_screen_result() {
        // Arrange
        let repository = Box::new(MockHighScoresRepository::empty());
        let manager = HighScoreManager::new(repository);
        let mut sut = HighScoresScreen::new(manager);
        let input_events = vec![InputEvent::Quit];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Quit);
    }

    #[test]
    fn handle_input_unknown_key_returns_continue() {
        // Arrange
        let repository = Box::new(MockHighScoresRepository::empty());
        let manager = HighScoreManager::new(repository);
        let mut sut = HighScoresScreen::new(manager);
        let input_events = vec![InputEvent::KeyPressed(Key::Space)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Continue);
    }
}
