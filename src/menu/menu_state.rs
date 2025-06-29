use crate::menu::MenuItem;

pub struct MenuState {
    items: Vec<MenuItem>,
    selected_index: usize,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            items: vec![
                MenuItem::StartGame,
                MenuItem::HighScores,
                MenuItem::Quit,
            ],
            selected_index: 0,
        }
    }

    pub fn get_items(&self) -> &[MenuItem] {
        &self.items
    }

    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn get_selected_item(&self) -> &MenuItem {
        &self.items[self.selected_index]
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.items.len() - 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.items.len() - 1 {
            self.selected_index += 1;
        } else {
            self.selected_index = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn new_menu_state_starts_with_first_item_selected() {
        // Act
        let sut = MenuState::new();

        // Assert
        assert_eq!(sut.get_selected_index(), 0);
        assert_eq!(sut.get_selected_item(), &MenuItem::StartGame);
    }

    #[test]
    fn move_down_advances_selection() {
        // Arrange
        let mut sut = MenuState::new();

        // Act
        sut.move_down();

        // Assert
        assert_eq!(sut.get_selected_index(), 1);
        assert_eq!(sut.get_selected_item(), &MenuItem::HighScores);
    }

    #[test]
    fn move_down_at_end_wraps_to_beginning() {
        // Arrange
        let mut sut = MenuState::new();
        sut.selected_index = 2; // Last item

        // Act
        sut.move_down();

        // Assert
        assert_eq!(sut.get_selected_index(), 0);
        assert_eq!(sut.get_selected_item(), &MenuItem::StartGame);
    }

    #[test]
    fn move_up_decreases_selection() {
        // Arrange
        let mut sut = MenuState::new();
        sut.selected_index = 1;

        // Act
        sut.move_up();

        // Assert
        assert_eq!(sut.get_selected_index(), 0);
        assert_eq!(sut.get_selected_item(), &MenuItem::StartGame);
    }

    #[test]
    fn move_up_at_beginning_wraps_to_end() {
        // Arrange
        let mut sut = MenuState::new();

        // Act
        sut.move_up();

        // Assert
        assert_eq!(sut.get_selected_index(), 2);
        assert_eq!(sut.get_selected_item(), &MenuItem::Quit);
    }

    #[rstest]
    #[case(0, MenuItem::StartGame)]
    #[case(1, MenuItem::HighScores)]
    #[case(2, MenuItem::Quit)]
    fn get_selected_item_returns_correct_item_for_index(
        #[case] index: usize,
        #[case] expected_item: MenuItem,
    ) {
        // Arrange
        let mut sut = MenuState::new();
        sut.selected_index = index;

        // Act
        let result = sut.get_selected_item();

        // Assert
        assert_eq!(result, &expected_item);
    }
}