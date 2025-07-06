use crate::constants::*;
use crate::game_logic::GameResult;
use crate::graphics::{Color, Display};
use crate::high_scores::{HighScore, HighScoreManager, HighScores, HighScoresScreenBackground};
use crate::input::{InputEvent, Key};
use crate::screens::{Screen, ScreenResult};
use std::time::Duration;

pub struct EnterHighScoreScreen {
    high_score_manager: HighScoreManager,
    background: HighScoresScreenBackground,
    preview_scores: HighScores,
    editing_index: usize,
    player_name: String,
    game_result: GameResult,
}

impl EnterHighScoreScreen {
    pub fn new(high_score_manager: HighScoreManager, game_result: GameResult) -> Self {
        let mut preview_scores = high_score_manager.get_high_scores().clone();
        let placeholder_score = HighScore::new("........".to_string(), game_result);
        let editing_index = preview_scores.add(placeholder_score);
        Self {
            high_score_manager,
            background: HighScoresScreenBackground::new(),
            preview_scores,
            editing_index,
            player_name: String::new(),
            game_result,
        }
    }

    fn get_scores_to_display(&self) -> Vec<(usize, String, u32, u32)> {
        self.preview_scores
            .get_scores()
            .iter()
            .enumerate()
            .map(|(i, score)| {
                let display_name = if i == self.editing_index {
                    format!("{:.<8}", self.player_name)
                } else {
                    score.name.clone()
                };
                (
                    i + 1,
                    display_name,
                    score.game_result.score,
                    score.game_result.level,
                )
            })
            .collect()
    }

    fn save_high_score(&mut self) -> Result<(), String> {
        let final_player_name = self.get_final_player_name();
        let high_score = HighScore::new(
            final_player_name,
            GameResult {
                score: self.game_result.score,
                level: self.game_result.level,
            },
        );
        self.high_score_manager.add_high_score(high_score)?;
        Ok(())
    }

    fn get_final_player_name(&mut self) -> String {
        if self.player_name.is_empty() {
            "PLAYER".to_string()
        } else {
            self.player_name.to_string()
        }
    }
}

impl Screen for EnterHighScoreScreen {
    fn update(&mut self, _delta_time: Duration) {
        // No updates needed yet
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

        let scores = self.get_scores_to_display();
        for (rank, name, score, level) in scores {
            let y = HIGH_SCORES_Y + (rank as u32 * HIGH_SCORES_LINE_HEIGHT);
            let text = format!("{:2}  {:06}  {:5}  {}", rank, score, level + 1, name);
            display.draw_text(&text, HIGH_SCORES_X, y, Color::WHITE)?;
        }

        display.draw_text("NEW HIGH SCORE! ENTER YOUR NAME.", 70, 320, Color::WHITE)?;

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
                        eprintln!("Failed to save high score: {e}");
                    }
                    return ScreenResult::ShowHighScores;
                }
                InputEvent::KeyPressed(Key::Alphanumeric(ch)) => {
                    if self.player_name.len() < 8 {
                        self.player_name.push(*ch);
                    }
                }
                InputEvent::KeyPressed(Key::Backspace) => {
                    self.player_name.pop();
                }
                _ => {}
            }
        }
        ScreenResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::graphics::MockDisplay;
    use crate::high_scores::MockHighScoresRepository;

    #[test]
    fn new_creates_enter_high_score_screen() {
        // Act
        let sut = create_test_screen();

        // Assert
        assert_eq!(sut.game_result.score, 1500);
        assert_eq!(sut.game_result.level, 3);
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

    #[test]
    fn get_display_scores_uses_existing_score_names_for_non_editing_entries() {
        // Arrange
        let mut existing_scores = HighScores::new();
        existing_scores.add(HighScore::new(
            "ALICE".to_string(),
            GameResult {
                score: 2000,
                level: 1,
            },
        ));
        existing_scores.add(HighScore::new(
            "BOB".to_string(),
            GameResult {
                score: 1000,
                level: 1,
            },
        ));

        let repository = Box::new(MockHighScoresRepository::new(existing_scores));
        let manager = HighScoreManager::new(repository);
        let sut = EnterHighScoreScreen::new(
            manager,
            GameResult {
                score: 1500,
                level: 3,
            },
        );

        // Act
        let display_scores = sut.get_scores_to_display();

        // Assert
        assert_eq!(display_scores[0].1, "ALICE"); // Uses existing score name
        assert_eq!(display_scores[2].1, "BOB"); // Uses existing score name
    }

    #[rstest]
    #[case("HERO", "HERO")]
    #[case("", "PLAYER")]
    fn get_final_player_name_returns_correct_name(
        #[case] input_name: &str,
        #[case] expected: &str,
    ) {
        // Arrange
        let mut sut = create_test_screen();
        sut.player_name = input_name.to_string();

        // Act
        let result = sut.get_final_player_name();

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn update_does_not_change_state() {
        // Arrange
        let mut sut = create_test_screen();
        let initial_player_name = sut.player_name.clone();
        let initial_editing_index = sut.editing_index;
        let initial_score = sut.game_result.score;

        // Act
        sut.update(Duration::from_millis(100));

        // Assert
        assert_eq!(sut.player_name, initial_player_name);
        assert_eq!(sut.editing_index, initial_editing_index);
        assert_eq!(sut.game_result.score, initial_score);
    }

    #[test]
    fn handle_input_quit_returns_quit_screen_result() {
        // Arrange
        let mut sut = create_test_screen();
        let input_events = vec![InputEvent::Quit];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Quit);
    }

    #[test]
    fn handle_input_enter_handles_save_error_gracefully() {
        // Arrange
        let mut failing_repository = Box::new(MockHighScoresRepository::empty());
        failing_repository.fail_on_save = true;
        let manager = HighScoreManager::new(failing_repository);
        let mut sut = EnterHighScoreScreen::new(
            manager,
            GameResult {
                score: 1500,
                level: 3,
            },
        );

        let input_events = vec![InputEvent::KeyPressed(Key::Enter)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::ShowHighScores); // Should still transition despite save error
    }

    #[test]
    fn handle_input_unknown_key_returns_continue() {
        // Arrange
        let mut sut = create_test_screen();
        let input_events = vec![InputEvent::KeyPressed(Key::Up)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Continue);
    }

    fn create_test_screen() -> EnterHighScoreScreen {
        let repository = Box::new(MockHighScoresRepository::empty());
        let manager = HighScoreManager::new(repository);
        EnterHighScoreScreen::new(
            manager,
            GameResult {
                score: 1500,
                level: 3,
            },
        )
    }

    #[test]
    fn handle_input_alphanumeric_adds_character_to_player_name() {
        // Arrange
        let mut sut = create_test_screen();
        let input_events = vec![
            InputEvent::KeyPressed(Key::Alphanumeric('S')),
            InputEvent::KeyPressed(Key::Alphanumeric('A')),
            InputEvent::KeyPressed(Key::Alphanumeric('M')),
        ];

        // Act
        for event in input_events {
            sut.handle_input(&[event]);
        }

        // Assert
        assert_eq!(sut.player_name, "SAM");
    }

    #[test]
    fn handle_input_backspace_on_empty_name_does_nothing() {
        // Arrange
        let mut sut = create_test_screen();
        // player_name is empty by default
        let input_events = vec![InputEvent::KeyPressed(Key::Backspace)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Continue);
        assert_eq!(sut.player_name, "");
    }

    #[test]
    fn handle_input_backspace_removes_last_character() {
        // Arrange
        let mut sut = create_test_screen();
        sut.player_name = "SAM".to_string();
        let input_events = vec![InputEvent::KeyPressed(Key::Backspace)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Continue);
        assert_eq!(sut.player_name, "SA");
    }
}
