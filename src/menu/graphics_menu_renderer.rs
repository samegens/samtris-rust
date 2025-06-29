use crate::constants::*;
use crate::graphics::{Color, Display};
use crate::menu::{Menu, MenuRenderer};

pub struct GraphicsMenuRenderer;

impl GraphicsMenuRenderer {
    pub fn new() -> Self {
        Self
    }

    fn draw_title<D: Display>(&self, display: &mut D) -> Result<(), String> {
        const TITLE: &str = "SAMTris";
        const TITLE_WIDTH: u32 = TITLE.len() as u32 * CHAR_WIDTH * 2; // Double size for title
        let title_x = (WINDOW_WIDTH_IN_BLOCKS * BLOCK_SIZE - TITLE_WIDTH) / 2;
        let title_y = WINDOW_HEIGHT_IN_BLOCKS * BLOCK_SIZE / 4;

        display.draw_text(TITLE, title_x, title_y, Color::CYAN)
    }

    fn draw_menu_items<D: Display>(&self, menu: &Menu, display: &mut D) -> Result<(), String> {
        let menu_start_y = WINDOW_HEIGHT_IN_BLOCKS * BLOCK_SIZE / 2;
        let line_height = CHAR_HEIGHT * 2;

        for (index, item) in menu.get_items().iter().enumerate() {
            let is_selected = index == menu.get_selected_index();
            let color = if is_selected {
                Color::YELLOW
            } else {
                Color::WHITE
            };

            let text = if is_selected {
                format!("> {}", item.display_text())
            } else {
                format!("  {}", item.display_text())
            };

            let text_width = text.len() as u32 * CHAR_WIDTH;
            let text_x = (WINDOW_WIDTH_IN_BLOCKS * BLOCK_SIZE - text_width) / 2;
            let text_y = menu_start_y + (index as u32 * line_height);

            display.draw_text(&text, text_x, text_y, color)?;
        }

        Ok(())
    }
}

impl MenuRenderer for GraphicsMenuRenderer {
    fn draw<D: Display>(&self, menu: &Menu, display: &mut D) -> Result<(), String> {
        self.draw_title(display)?;
        self.draw_menu_items(menu, display)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::MockDisplay;

    #[test]
    fn graphics_menu_renderer_draws_title() {
        // Arrange
        let sut = GraphicsMenuRenderer::new();
        let menu = Menu::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&menu, &mut display);

        // Assert
        assert!(result.is_ok());
        let title_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text == "SAMTris");
        assert!(title_drawn);
    }

    #[test]
    fn graphics_menu_renderer_draws_all_menu_items() {
        // Arrange
        let sut = GraphicsMenuRenderer::new();
        let menu = Menu::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&menu, &mut display);

        // Assert
        assert!(result.is_ok());

        let start_game_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text.contains("START GAME"));
        assert!(start_game_drawn);

        let high_scores_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text.contains("HIGH SCORES"));
        assert!(high_scores_drawn);

        let quit_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text.contains("QUIT"));
        assert!(quit_drawn);
    }

    #[test]
    fn graphics_menu_renderer_highlights_selected_item() {
        // Arrange
        let sut = GraphicsMenuRenderer::new();
        let menu = Menu::new(); // First item selected by default
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&menu, &mut display);

        // Assert
        assert!(result.is_ok());

        let selected_item_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, color)| text.starts_with('>') && *color == Color::YELLOW);
        assert!(selected_item_drawn);
    }
}
