use crate::common::{Position, RotationIndex};
use crate::constants::*;
use crate::graphics::{Color, Display, HudRenderer, HudView};
use crate::tetromino::TetrominoDefinitions;

pub struct GraphicsHudRenderer;

impl GraphicsHudRenderer {
    pub fn new() -> Self {
        Self
    }

    fn draw_level<D: Display + ?Sized>(
        &self,
        hud_view: &HudView,
        display: &mut D,
    ) -> Result<(), String> {
        let level = hud_view.current_level + 1; // Display as 1-based
        display.draw_text(
            &format!("Level: {level}"),
            LEVEL_OFFSET_X,
            LEVEL_OFFSET_Y,
            Color::WHITE,
        )
    }

    fn draw_lines_cleared<D: Display + ?Sized>(
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

    fn draw_game_over<D: Display + ?Sized>(&self, display: &mut D) -> Result<(), String> {
        let x: u32 = (PLAYFIELD_OFFSET_X as i32
            + (PLAYFIELD_WIDTH as i32 * BLOCK_SIZE as i32 - GAME_OVER_WIDTH as i32) / 2)
            as u32;
        let y: u32 = (PLAYFIELD_OFFSET_Y as i32
            + (PLAYFIELD_HEIGHT as i32 * BLOCK_SIZE as i32 - GAME_OVER_HEIGHT as i32) / 2)
            as u32;

        // Draw red background rectangle
        display.draw_rectangle(x, y, GAME_OVER_WIDTH, GAME_OVER_HEIGHT, Color::RED)?;

        // Calculate text position based on string length and character width
        let text = "GAME OVER";
        let text_width = text.len() as u32 * CHAR_WIDTH;
        let text_x = x + (GAME_OVER_WIDTH - text_width) / 2;
        let text_y = y + (GAME_OVER_HEIGHT - CHAR_HEIGHT) / 2;

        display.draw_text(text, text_x, text_y, Color::WHITE)
    }

    fn draw_widget_for_next_tetromino<D: Display + ?Sized>(
        &self,
        hud_view: &HudView,
        display: &mut D,
    ) -> Result<(), String> {
        draw_label_for_next_tetromino(display)?;
        draw_border_for_next_tetromino(display)?;
        self.draw_tetromino_for_next_tetromino(hud_view, display)
    }

    fn draw_tetromino_for_next_tetromino<D: Display + ?Sized>(
        &self,
        hud_view: &HudView,
        display: &mut D,
    ) -> Result<(), String> {
        if hud_view.show_game_over {
            return Ok(());
        }

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

    fn draw_score<D: Display + ?Sized>(
        &self,
        hud_view: &HudView,
        display: &mut D,
    ) -> Result<(), String> {
        display.draw_text(
            &format!("Score: {}", hud_view.score),
            SCORE_OFFSET_X,
            SCORE_OFFSET_Y,
            Color::WHITE,
        )
    }
}

fn draw_label_for_next_tetromino<D: Display + ?Sized>(display: &mut D) -> Result<(), String> {
    const TEXT: &str = "NEXT";
    const TEXT_WIDTH: u32 = TEXT.len() as u32 * CHAR_WIDTH;
    let x = NEXT_TETROMINO_OFFSET_X + (NEXT_TETROMINO_AREA_WIDTH - TEXT_WIDTH) / 2;
    let y = NEXT_TETROMINO_OFFSET_Y - CHAR_HEIGHT;
    display.draw_text(TEXT, x, y, Color::WHITE)
}

fn draw_border_for_next_tetromino<D: Display + ?Sized>(display: &mut D) -> Result<(), String> {
    let border_color = Color::WHITE;
    display.draw_rectangle(
        NEXT_TETROMINO_OFFSET_X - 1,
        NEXT_TETROMINO_OFFSET_Y - 1,
        NEXT_TETROMINO_AREA_WIDTH + 2,
        1,
        border_color,
    )?;
    display.draw_rectangle(
        NEXT_TETROMINO_OFFSET_X - 1,
        NEXT_TETROMINO_OFFSET_Y + NEXT_TETROMINO_AREA_HEIGHT,
        NEXT_TETROMINO_AREA_WIDTH + 2,
        1,
        border_color,
    )?;
    display.draw_rectangle(
        NEXT_TETROMINO_OFFSET_X - 1,
        NEXT_TETROMINO_OFFSET_Y - 1,
        1,
        NEXT_TETROMINO_AREA_HEIGHT + 2,
        border_color,
    )?;
    display.draw_rectangle(
        NEXT_TETROMINO_OFFSET_X + NEXT_TETROMINO_AREA_WIDTH,
        NEXT_TETROMINO_OFFSET_Y - 1,
        1,
        NEXT_TETROMINO_AREA_HEIGHT + 2,
        border_color,
    )
}

impl HudRenderer for GraphicsHudRenderer {
    fn draw<D: Display + ?Sized>(&self, hud_view: &HudView, display: &mut D) -> Result<(), String> {
        self.draw_score(hud_view, display)?;
        self.draw_lines_cleared(hud_view, display)?;
        self.draw_level(hud_view, display)?;
        self.draw_widget_for_next_tetromino(hud_view, display)?;

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
        assert!(next_text_drawn);
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
        assert!(level_text_drawn);
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
        assert!(lines_text_drawn);
    }

    #[test]
    fn hud_renderer_draws_game_over_when_flag_set() {
        // Arrange
        let sut = GraphicsHudRenderer::new();
        let hud_view = HudView {
            next_tetromino_type: TetrominoType::O,
            current_level: 2,
            total_lines_cleared: 8,
            score: 500,
            show_game_over: true,
        };
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&hud_view, &mut display);

        // Assert
        assert!(result.is_ok());

        let game_over_rectangle_drawn = display
            .drawn_rectangles
            .iter()
            .any(|(_, _, _, _, color)| *color == Color::RED);
        assert!(game_over_rectangle_drawn);

        let game_over_text_drawn = display
            .drawn_text
            .iter()
            .any(|(text, _, _, _)| text == "GAME OVER");
        assert!(game_over_text_drawn);
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
        assert!(score_text_drawn);
    }

    #[test]
    fn draw_does_not_draw_next_area_during_game_over() {
        // Arrange
        let sut = GraphicsHudRenderer::new();
        let hud_view = HudView {
            next_tetromino_type: TetrominoType::T,
            current_level: 1,
            total_lines_cleared: 5,
            score: 200,
            show_game_over: true,
        };
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&hud_view, &mut display);

        // Assert
        assert!(result.is_ok());

        let tetromino_blocks_drawn = display
            .drawn_blocks
            .iter()
            .any(|(_, tetromino_type)| *tetromino_type == TetrominoType::T);
        assert!(!tetromino_blocks_drawn);
    }
}
