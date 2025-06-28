use crate::common::{Position, RotationIndex};
use crate::constants::*;
use crate::graphics::{Color, Display, HudRenderer, HudView};
use crate::tetromino::TetrominoDefinitions;

pub struct GraphicsHudRenderer;

impl GraphicsHudRenderer {
    pub fn new() -> Self {
        Self
    }

    fn draw_level<D: Display>(&self, hud_view: &HudView, display: &mut D) -> Result<(), String> {
        let level = hud_view.current_level + 1; // Display as 1-based
        display.draw_text(
            &format!("Level: {level}"),
            LEVEL_OFFSET_X,
            LEVEL_OFFSET_Y,
            Color::WHITE,
        )
    }

    fn draw_lines_cleared<D: Display>(
        &self,
        hud_view: &HudView,
        display: &mut D,
    ) -> Result<(), String> {
        display.draw_text(
            &format!("Lines: {}", hud_view.total_lines_cleared),
            LINES_OFFSET_X,
            LINES_OFFSET_Y,
            Color::WHITE,
        )
    }

    fn draw_game_over<D: Display>(&self, display: &mut D) -> Result<(), String> {
        let x = PLAYFIELD_OFFSET_X + (PLAYFIELD_WIDTH * BLOCK_SIZE - GAME_OVER_WIDTH) / 2;
        let y = PLAYFIELD_OFFSET_Y + (PLAYFIELD_HEIGHT * BLOCK_SIZE - GAME_OVER_HEIGHT) / 2;

        display.draw_rectangle(x, y, GAME_OVER_WIDTH, GAME_OVER_HEIGHT, Color::RED)
    }

    fn draw_next_tetromino_area<D: Display>(
        &self,
        hud_view: &HudView,
        display: &mut D,
    ) -> Result<(), String> {
        const NEXT_TEXT_OFFSET_X: u32 = 12;
        const NEXT_TEXT_OFFSET_Y: u32 = 20;
        display.draw_text(
            "NEXT",
            NEXT_TETROMINO_OFFSET_X + NEXT_TEXT_OFFSET_X,
            NEXT_TETROMINO_OFFSET_Y - NEXT_TEXT_OFFSET_Y,
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

    fn draw_score<D: Display>(&self, hud_view: &HudView, display: &mut D) -> Result<(), String> {
        display.draw_text(
            &format!("Score: {}", hud_view.score),
            SCORE_OFFSET_X,
            SCORE_OFFSET_Y,
            Color::WHITE,
        )
    }
}

impl HudRenderer for GraphicsHudRenderer {
    fn draw<D: Display>(&self, hud_view: &HudView, display: &mut D) -> Result<(), String> {
        self.draw_score(hud_view, display)?;
        self.draw_lines_cleared(hud_view, display)?;
        self.draw_level(hud_view, display)?;
        self.draw_next_tetromino_area(hud_view, display)?;

        if hud_view.show_game_over {
            self.draw_game_over(display)?;
        }

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
            current_level: 1,
            total_lines_cleared: 0,
            score: 0,
            show_game_over: false,
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

    #[test]
    fn hud_renderer_draws_level_text() {
        // Arrange
        let sut = GraphicsHudRenderer::new();
        let hud_view = HudView {
            next_tetromino_type: TetrominoType::T,
            current_level: 3,
            total_lines_cleared: 25,
            score: 0,
            show_game_over: false,
        };
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&hud_view, &mut display);

        // Assert
        assert!(result.is_ok());
        let level_text_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text == "Level: 4"); // current_level + 1
        assert!(level_text_drawn, "Level text should be drawn");
    }

    #[test]
    fn hud_renderer_draws_lines_text() {
        // Arrange
        let sut = GraphicsHudRenderer::new();
        let hud_view = HudView {
            next_tetromino_type: TetrominoType::J,
            current_level: 1,
            total_lines_cleared: 15,
            score: 0,
            show_game_over: false,
        };
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&hud_view, &mut display);

        // Assert
        assert!(result.is_ok());
        let lines_text_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text == "Lines: 15");
        assert!(lines_text_drawn, "Lines text should be drawn");
    }

    #[test]
    fn hud_renderer_draws_game_over_when_flag_set() {
        // Arrange
        let sut = GraphicsHudRenderer::new();
        let hud_view = HudView {
            next_tetromino_type: TetrominoType::O,
            current_level: 2,
            total_lines_cleared: 8,
            score: 0,
            show_game_over: true,
        };
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&hud_view, &mut display);

        // Assert
        assert!(result.is_ok());
        let game_over_drawn = display
            .drawn_rectangles
            .iter()
            .any(|(_, _, _, _, color)| *color == Color::RED);
        assert!(game_over_drawn, "Game over rectangle should be drawn");
    }

    #[test]
    fn hud_renderer_draws_score_text() {
        // Arrange
        let sut = GraphicsHudRenderer::new();
        let hud_view = HudView {
            next_tetromino_type: TetrominoType::I,
            current_level: 2,
            total_lines_cleared: 8,
            score: 1240,
            show_game_over: false,
        };
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&hud_view, &mut display);

        // Assert
        assert!(result.is_ok());
        let score_text_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text == "Score: 1240");
        assert!(score_text_drawn, "Score text should be drawn");
    }
}
