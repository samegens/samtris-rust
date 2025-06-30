use crate::common::Position;
use crate::constants::*;
use crate::graphics::{Color, Display};
use crate::menu::{Menu, MenuRenderer, MenuTitle};
use crate::tetromino::TetrominoType;

pub struct GraphicsMenuRenderer {
    title: MenuTitle,
}

impl GraphicsMenuRenderer {
    pub fn new() -> Self {
        Self {
            title: MenuTitle::new(),
        }
    }

    fn draw_menu_items<D: Display + ?Sized>(
        &self,
        menu: &Menu,
        display: &mut D,
    ) -> Result<(), String> {
        let menu_start_y = WINDOW_HEIGHT_IN_BLOCKS * BLOCK_SIZE * 3 / 5; // Lower to make room for tetromino title
        let line_height = CHAR_HEIGHT * 2;

        for (index, item) in menu.get_items().iter().enumerate() {
            let is_selected = index == menu.get_selected_index();
            let color = Color::WHITE;

            let text_x = WINDOW_WIDTH_IN_BLOCKS * BLOCK_SIZE / 3;
            let text_y = menu_start_y + (index as u32 * line_height);

            if is_selected {
                let block_pos = Position::new((text_x - BLOCK_SIZE * 2) as i32, text_y as i32);
                display.draw_block(block_pos, TetrominoType::O)?;
            }

            display.draw_text(item.display_text(), text_x, text_y, color)?;
        }

        Ok(())
    }

    fn draw_subtitle<D: Display + ?Sized>(&self, display: &mut D) -> Result<(), String> {
        const SUBTITLE: &str = "- RUST EDITION -";
        let subtitle_width = SUBTITLE.len() as u32 * CHAR_WIDTH;
        let subtitle_x = (WINDOW_WIDTH_IN_BLOCKS * BLOCK_SIZE - subtitle_width) / 2;
        let subtitle_y = WINDOW_HEIGHT_IN_BLOCKS * BLOCK_SIZE / 6 + (7 * BLOCK_SIZE);

        display.draw_text(SUBTITLE, subtitle_x, subtitle_y, Color::WHITE)
    }
}

impl MenuRenderer for GraphicsMenuRenderer {
    fn draw<D: Display + ?Sized>(&self, menu: &Menu, display: &mut D) -> Result<(), String> {
        self.title.draw(display)?;
        self.draw_subtitle(display)?;
        self.draw_menu_items(menu, display)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::MockDisplay;

    #[test]
    fn graphics_menu_renderer_draws_tetromino_title() {
        // Arrange
        let sut = GraphicsMenuRenderer::new();
        let menu = Menu::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&menu, &mut display);

        // Assert
        assert!(result.is_ok());
        assert!(
            !display.drawn_blocks.is_empty(),
            "Should draw tetromino blocks for title"
        );
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
            .any(|(text, _, _, _)| text.contains("PLAY"));
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

        assert!(!display.drawn_blocks.is_empty());
    }
}
