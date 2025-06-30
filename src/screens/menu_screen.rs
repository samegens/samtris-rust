use crate::graphics::Display;
use crate::input::{InputEvent, Key};
use crate::menu::{GraphicsMenuRenderer, Menu, MenuItem, MenuRenderer};
use crate::screens::ScreenResult;
use std::time::Duration;

pub struct MenuScreen {
    menu: Menu,
    menu_renderer: GraphicsMenuRenderer,
}

impl MenuScreen {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(),
            menu_renderer: GraphicsMenuRenderer::new(),
        }
    }

    pub fn update(&mut self, _delta_time: Duration) {
        // Menu doesn't need time-based updates for now
    }

    pub fn draw<D: Display>(&mut self, display: &mut D) -> Result<(), String> {
        display.clear()?;
        self.menu_renderer.draw(&self.menu, display)?;
        display.present()?;
        Ok(())
    }

    pub fn handle_input(&mut self, input_events: &[InputEvent]) -> ScreenResult {
        for event in input_events {
            match event {
                InputEvent::Quit => return ScreenResult::Quit,
                InputEvent::KeyPressed(key) => {
                    if let Some(result) = self.handle_key_press(*key) {
                        return result;
                    }
                }
                InputEvent::KeyReleased(_) => {
                    // Menu doesn't need key release events
                }
            }
        }
        ScreenResult::Continue
    }

    fn handle_key_press(&mut self, key: Key) -> Option<ScreenResult> {
        match key {
            Key::Up => {
                self.menu.select_previous_item();
                None
            }
            Key::Down => {
                self.menu.select_next_item();
                None
            }
            Key::Enter | Key::Space => Some(self.activate_selected_item()),
            Key::Escape => Some(ScreenResult::Quit),
            _ => None,
        }
    }

    fn activate_selected_item(&self) -> ScreenResult {
        match self.menu.get_selected_item() {
            MenuItem::Play => ScreenResult::Play,
            MenuItem::HighScores => ScreenResult::ShowHighScores,
            MenuItem::Quit => ScreenResult::Quit,
        }
    }

    #[cfg(test)]
    pub fn get_menu(&self) -> &Menu {
        &self.menu
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::MockDisplay;
    use crate::menu::MenuItem;
    use rstest::rstest;

    #[test]
    fn new_menu_screen_starts_with_play_selected() {
        // Act
        let sut = MenuScreen::new();

        // Assert
        assert_eq!(sut.get_menu().get_selected_item(), &MenuItem::Play);
        assert_eq!(sut.get_menu().get_selected_index(), 0);
    }

    #[test]
    fn menu_screen_can_draw() {
        // Arrange
        let mut sut = MenuScreen::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.cleared);
        assert!(display.presented);
        assert!(!display.drawn_text.is_empty());
    }

    #[test]
    fn update_does_not_change_menu_state() {
        // Arrange
        let mut sut = MenuScreen::new();
        let initial_selection = sut.get_menu().get_selected_index();

        // Act
        sut.update(Duration::from_millis(100));

        // Assert
        assert_eq!(sut.get_menu().get_selected_index(), initial_selection);
    }

    #[test]
    fn handle_input_quit_returns_quit_screen_result() {
        // Arrange
        let mut sut = MenuScreen::new();
        let input_events = vec![InputEvent::Quit];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Quit);
    }

    #[test]
    fn handle_input_key_released_does_nothing() {
        // Arrange
        let mut sut = MenuScreen::new();
        let initial_selection = sut.get_menu().get_selected_index();
        let input_events = vec![InputEvent::KeyReleased(Key::Up)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Continue);
        assert_eq!(sut.get_menu().get_selected_index(), initial_selection);
    }

    #[rstest]
    #[case(Key::Up, 2)] // Should wrap to last item (Quit)
    #[case(Key::Down, 1)] // Should advance to next item (HighScores)
    fn handle_input_navigation_keys_change_selection(
        #[case] key: Key,
        #[case] expected_index: usize,
    ) {
        // Arrange
        let mut sut = MenuScreen::new();
        let input_events = vec![InputEvent::KeyPressed(key)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Continue);
        assert_eq!(sut.get_menu().get_selected_index(), expected_index);
    }

    #[rstest]
    #[case(MenuItem::Play, ScreenResult::Play)]
    #[case(MenuItem::HighScores, ScreenResult::ShowHighScores)]
    #[case(MenuItem::Quit, ScreenResult::Quit)]
    fn handle_input_enter_activates_selected_item(
        #[case] menu_item: MenuItem,
        #[case] expected_result: ScreenResult,
    ) {
        // Arrange
        let mut sut = MenuScreen::new();

        // Navigate to the desired menu item
        while sut.get_menu().get_selected_item() != &menu_item {
            sut.menu.select_next_item();
        }

        let input_events = vec![InputEvent::KeyPressed(Key::Enter)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(MenuItem::Play, ScreenResult::Play)]
    #[case(MenuItem::HighScores, ScreenResult::ShowHighScores)]
    #[case(MenuItem::Quit, ScreenResult::Quit)]
    fn handle_input_space_activates_selected_item(
        #[case] menu_item: MenuItem,
        #[case] expected_result: ScreenResult,
    ) {
        // Arrange
        let mut sut = MenuScreen::new();

        // Navigate to the desired menu item
        while sut.get_menu().get_selected_item() != &menu_item {
            sut.menu.select_next_item();
        }

        let input_events = vec![InputEvent::KeyPressed(Key::Space)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, expected_result);
    }

    #[test]
    fn handle_input_escape_returns_quit() {
        // Arrange
        let mut sut = MenuScreen::new();
        let input_events = vec![InputEvent::KeyPressed(Key::Escape)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Quit);
    }

    #[test]
    fn handle_input_unknown_key_does_nothing() {
        // Arrange
        let mut sut = MenuScreen::new();
        let initial_selection = sut.get_menu().get_selected_index();
        let input_events = vec![InputEvent::KeyPressed(Key::Left)];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Continue);
        assert_eq!(sut.get_menu().get_selected_index(), initial_selection);
    }

    #[test]
    fn handle_input_multiple_events_processes_all() {
        // Arrange
        let mut sut = MenuScreen::new();
        let input_events = vec![
            InputEvent::KeyPressed(Key::Down), // Move to HighScores
            InputEvent::KeyPressed(Key::Down), // Move to Quit
        ];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Continue);
        assert_eq!(sut.get_menu().get_selected_item(), &MenuItem::Quit);
    }

    #[test]
    fn handle_input_stops_processing_on_screen_transition() {
        // Arrange
        let mut sut = MenuScreen::new();
        let input_events = vec![
            InputEvent::KeyPressed(Key::Enter), // Should activate Play and return
            InputEvent::KeyPressed(Key::Down),  // Should not be processed
        ];

        // Act
        let result = sut.handle_input(&input_events);

        // Assert
        assert_eq!(result, ScreenResult::Play);
        assert_eq!(sut.get_menu().get_selected_item(), &MenuItem::Play); // Should still be on Play
    }
}
