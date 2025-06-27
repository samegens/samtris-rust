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
        let area_width = NEXT_TETROMINO_AREA_WIDTH * BLOCK_SIZE;
        let area_height = NEXT_TETROMINO_AREA_HEIGHT * BLOCK_SIZE;

        // Top border
        display.draw_rectangle(
            NEXT_TETROMINO_OFFSET_X - 1,
            NEXT_TETROMINO_OFFSET_Y - 1,
            area_width + 2,
            1,
            border_color,
        )?;

        // Bottom border
        display.draw_rectangle(
            NEXT_TETROMINO_OFFSET_X - 1,
            NEXT_TETROMINO_OFFSET_Y + area_height,
            area_width + 2,
            1,
            border_color,
        )?;

        // Left border
        display.draw_rectangle(
            NEXT_TETROMINO_OFFSET_X - 1,
            NEXT_TETROMINO_OFFSET_Y - 1,
            1,
            area_height + 2,
            border_color,
        )?;

        // Right border
        display.draw_rectangle(
            NEXT_TETROMINO_OFFSET_X + area_width,
            NEXT_TETROMINO_OFFSET_Y - 1,
            1,
            area_height + 2,
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

        let area_center_x = NEXT_TETROMINO_OFFSET_X + (NEXT_TETROMINO_AREA_WIDTH * BLOCK_SIZE) / 2;
        let area_center_y = NEXT_TETROMINO_OFFSET_Y + (NEXT_TETROMINO_AREA_HEIGHT * BLOCK_SIZE) / 2;

        // Offset to center the tetromino in the preview area (assuming 4x4 tetromino grid)
        let tetromino_center_offset_x = 2 * BLOCK_SIZE as i32;
        let tetromino_center_offset_y = 2 * BLOCK_SIZE as i32;

        let preview_base_position = Position::new(
            area_center_x as i32 - tetromino_center_offset_x,
            area_center_y as i32 - tetromino_center_offset_y,
        );

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
