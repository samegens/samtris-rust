use crate::menu::MenuItem;

pub struct Menu {
    items: Vec<MenuItem>,
    selected_index: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            items: vec![MenuItem::Play, MenuItem::HighScores, MenuItem::Quit],
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

    pub fn select_previous_item(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.items.len() - 1;
        }
    }

    pub fn select_next_item(&mut self) {
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
    fn new_menu_starts_with_first_item_selected() {
        // Act
        let sut = Menu::new();

        // Assert
        assert_eq!(sut.get_selected_index(), 0);
        assert_eq!(sut.get_selected_item(), &MenuItem::Play);
    }

    #[test]
    fn select_next_item_advances_selection() {
        // Arrange
        let mut sut = Menu::new();

        // Act
        sut.select_next_item();

        // Assert
        assert_eq!(sut.get_selected_index(), 1);
        assert_eq!(sut.get_selected_item(), &MenuItem::HighScores);
    }

    #[test]
    fn select_next_item_at_end_wraps_to_beginning() {
        // Arrange
        let mut sut = Menu::new();
        sut.selected_index = 2; // Last item

        // Act
        sut.select_next_item();

        // Assert
        assert_eq!(sut.get_selected_index(), 0);
        assert_eq!(sut.get_selected_item(), &MenuItem::Play);
    }

    #[test]
    fn select_previous_item_decreases_selection() {
        // Arrange
        let mut sut = Menu::new();
        sut.selected_index = 1;

        // Act
        sut.select_previous_item();

        // Assert
        assert_eq!(sut.get_selected_index(), 0);
        assert_eq!(sut.get_selected_item(), &MenuItem::Play);
    }

    #[test]
    fn select_previous_item_at_beginning_wraps_to_end() {
        // Arrange
        let mut sut = Menu::new();

        // Act
        sut.select_previous_item();

        // Assert
        assert_eq!(sut.get_selected_index(), 2);
        assert_eq!(sut.get_selected_item(), &MenuItem::Quit);
    }

    #[rstest]
    #[case(0, MenuItem::Play)]
    #[case(1, MenuItem::HighScores)]
    #[case(2, MenuItem::Quit)]
    fn get_selected_item_returns_correct_item_for_index(
        #[case] index: usize,
        #[case] expected_item: MenuItem,
    ) {
        // Arrange
        let mut sut = Menu::new();
        sut.selected_index = index;

        // Act
        let result = sut.get_selected_item();

        // Assert
        assert_eq!(result, &expected_item);
    }
}
