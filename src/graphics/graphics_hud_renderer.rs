use crate::common::{Position, RotationIndex};
use crate::constants::*;
use crate::graphics::{Color, Display, HudRenderer, HudView};
use crate::tetromino::TetrominoDefinitions;

pub struct GraphicsHudRenderer;

impl GraphicsHudRenderer {
    pub fn new() -> Self {
        Self
    }

    fn draw_next_tetromino_area<D: Display>(
        &self,
        hud_view: &HudView,
        display: &mut D,
    ) -> Result<(), String> {
        // Draw "NEXT" label
        display.draw_text(
            "NEXT",
            NEXT_TETROMINO_OFFSET_X,
            NEXT_TETROMINO_OFFSET_Y - 20,
            Color::WHITE,
        )?;

        // Draw border around next tetromino area
        let border_color = Color::WHITE;

        // Top border
        display.draw_rectangle(
            NEXT_TETROMINO_OFFSET_X - 1,
            NEXT_TETROMINO_OFFSET_Y - 1,
            NEXT_TETROMINO_AREA_WIDTH + 2,
            1,
            border_color,
        )?;

        // Bottom border
        display.draw_rectangle(
            NEXT_TETROMINO_OFFSET_X - 1,
            NEXT_TETROMINO_OFFSET_Y + NEXT_TETROMINO_AREA_HEIGHT,
            NEXT_TETROMINO_AREA_WIDTH + 2,
            1,
            border_color,
        )?;

        // Left border
        display.draw_rectangle(
            NEXT_TETROMINO_OFFSET_X - 1,
            NEXT_TETROMINO_OFFSET_Y - 1,
            1,
            NEXT_TETROMINO_AREA_HEIGHT + 2,
            border_color,
        )?;

        // Right border
        display.draw_rectangle(
            NEXT_TETROMINO_OFFSET_X + NEXT_TETROMINO_AREA_WIDTH,
            NEXT_TETROMINO_OFFSET_Y - 1,
            1,
            NEXT_TETROMINO_AREA_HEIGHT + 2,
            border_color,
        )?;

        // Draw the next tetromino
        self.draw_next_tetromino_preview(hud_view, display)?;

        Ok(())
    }

    fn draw_next_tetromino_preview<D: Display>(
        &self,
        hud_view: &HudView,
        display: &mut D,
    ) -> Result<(), String> {
        let tetromino_definitions = TetrominoDefinitions::new();
        let definition = tetromino_definitions.get(hud_view.next_tetromino_type);
        let rotation = RotationIndex::new(0, definition.get_nr_rotations());
        let block_positions = definition.get_block_positions(rotation);

        let min_x = block_positions.iter().map(|pos| pos.x).min().unwrap_or(0);
        let max_x = block_positions.iter().map(|pos| pos.x).max().unwrap_or(0);
        let min_y = block_positions.iter().map(|pos| pos.y).min().unwrap_or(0);
        let max_y = block_positions.iter().map(|pos| pos.y).max().unwrap_or(0);

        let tetromino_width_px = (max_x - min_x + 1) * BLOCK_SIZE as i32;
        let tetromino_height_px = (max_y - min_y + 1) * BLOCK_SIZE as i32;

        // Center the tetromino within the total area
        let offset_x_px = (NEXT_TETROMINO_AREA_WIDTH as i32 - tetromino_width_px) / 2
            - (min_x * BLOCK_SIZE as i32);
        let offset_y_px = (NEXT_TETROMINO_AREA_HEIGHT as i32 - tetromino_height_px) / 2
            - (min_y * BLOCK_SIZE as i32);

        let preview_base_position = Position::new(
            NEXT_TETROMINO_OFFSET_X as i32 + offset_x_px,
            NEXT_TETROMINO_OFFSET_Y as i32 + offset_y_px,
        );

        // Draw each block of the tetromino
        for block_position in block_positions {
            let window_position = preview_base_position + block_position.scale(BLOCK_SIZE as i32);
            display.draw_block(window_position, hud_view.next_tetromino_type)?;
        }
        Ok(())
    }
}

impl HudRenderer for GraphicsHudRenderer {
    fn draw<D: Display>(&self, hud_view: &HudView, display: &mut D) -> Result<(), String> {
        self.draw_next_tetromino_area(hud_view, display)?;

        //TODO
        // self.draw_level_and_lines(hud_view, display)?;

        // if hud_view.show_game_over {
        //     self.draw_game_over(display)?;
        // }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::MockDisplay;
    use crate::tetromino::TetrominoType;

    use super::*;

    #[test]
    fn hud_renderer_draws_next_text() {
        // Arrange
        let sut = GraphicsHudRenderer::new();
        let hud_view = HudView {
            next_tetromino_type: TetrominoType::J,
        };
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&hud_view, &mut display);

        // Assert
        assert!(result.is_ok());
        let next_text_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text == "NEXT");
        assert!(next_text_drawn, "NEXT text should be drawn");
    }
}
