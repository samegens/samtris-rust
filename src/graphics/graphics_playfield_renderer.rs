use crate::common::Position;
use crate::constants::*;
use crate::graphics::{Color, Display, PlayfieldRenderer, PlayfieldView};
use crate::tetromino::TetrominoInstance;

pub struct GraphicsPlayfieldRenderer;

impl GraphicsPlayfieldRenderer {
    pub fn new() -> Self {
        Self
    }

    fn draw_border<D: Display>(&self, display: &mut D) -> Result<(), String> {
        let border_color = Color::WHITE;

        // Left border
        let mut x = PLAYFIELD_OFFSET_X - PLAYFIELD_BORDER_WIDTH;
        let mut y = PLAYFIELD_OFFSET_Y;
        let mut width = PLAYFIELD_BORDER_WIDTH;
        let mut height = PLAYFIELD_HEIGHT * BLOCK_SIZE;
        display.draw_rectangle(x, y, width, height, border_color)?;

        // Bottom border
        x = PLAYFIELD_OFFSET_X - PLAYFIELD_BORDER_WIDTH;
        y = PLAYFIELD_OFFSET_Y + PLAYFIELD_HEIGHT * BLOCK_SIZE;
        width = PLAYFIELD_BORDER_WIDTH + PLAYFIELD_WIDTH * BLOCK_SIZE + PLAYFIELD_BORDER_WIDTH;
        height = PLAYFIELD_BORDER_WIDTH;
        display.draw_rectangle(x, y, width, height, border_color)?;

        // Right border
        x = PLAYFIELD_OFFSET_X + PLAYFIELD_WIDTH * BLOCK_SIZE;
        y = PLAYFIELD_OFFSET_Y;
        width = PLAYFIELD_BORDER_WIDTH;
        height = PLAYFIELD_HEIGHT * BLOCK_SIZE;
        display.draw_rectangle(x, y, width, height, border_color)?;

        Ok(())
    }

    fn draw_playfield_blocks<D: Display>(
        &self,
        playfield_view: &PlayfieldView,
        blinking_lines: &[u32],
        show_blinking_lines: bool,
        display: &mut D,
    ) -> Result<(), String> {
        let x = PLAYFIELD_OFFSET_X as i32;
        let y = PLAYFIELD_OFFSET_Y as i32;
        let playfield_position = Position::new(x, y);

        for y in 0..playfield_view.dimensions.height {
            if !show_blinking_lines && blinking_lines.contains(&y) {
                continue; // Skip drawing this line if blinking lines are hidden
            }
            for x in 0..playfield_view.dimensions.width {
                let position = Position::new(x as i32, y as i32);
                self.draw_playfield_position(
                    playfield_view,
                    display,
                    playfield_position,
                    position,
                )?;
            }
        }

        Ok(())
    }

    fn draw_playfield_position<D: Display>(
        &self,
        playfield_view: &PlayfieldView,
        display: &mut D,
        playfield_position: Position,
        position: Position,
    ) -> Result<(), String> {
        if playfield_view.is_position_occupied(position) {
            if let Some(tetromino_type) = playfield_view.get_tetromino_type_at(position) {
                let window_position = playfield_position + position.scale(BLOCK_SIZE as i32);
                display.draw_block(window_position, tetromino_type)?;
            }
        }
        Ok(())
    }

    fn draw_current_tetromino<D: Display>(
        &self,
        current_tetromino: Option<&TetrominoInstance>,
        display: &mut D,
    ) -> Result<(), String> {
        let x = PLAYFIELD_OFFSET_X as i32;
        let y = PLAYFIELD_OFFSET_Y as i32;
        let playfield_position = Position::new(x, y);

        if let Some(tetromino) = current_tetromino {
            let blocks = tetromino.get_world_blocks();
            let tetromino_type = tetromino.get_type();

            for position in blocks {
                let window_position = playfield_position + position.scale(BLOCK_SIZE as i32);
                display.draw_block(window_position, tetromino_type)?;
            }
        }
        Ok(())
    }
}

impl PlayfieldRenderer for GraphicsPlayfieldRenderer {
    fn draw<D: Display>(
        &self,
        playfield_view: &PlayfieldView,
        current_tetromino: Option<&TetrominoInstance>,
        blinking_lines: &[u32],
        show_blinking_lines: bool,
        display: &mut D,
    ) -> Result<(), String> {
        self.draw_border(display)?;
        self.draw_playfield_blocks(playfield_view, blinking_lines, show_blinking_lines, display)?;
        self.draw_current_tetromino(current_tetromino, display)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Dimensions;
    use crate::graphics::MockDisplay;
    use crate::playfield::Playfield;
    use crate::tetromino::FixedTetrominoGenerator;
    use crate::tetromino::TetrominoDefinitions;
    use crate::tetromino::TetrominoInstance;
    use crate::tetromino::TetrominoType;

    #[test]
    fn draw_with_no_current_tetromino_only_draws_border() {
        // Arrange
        let playfield = create_test_playfield();
        let sut = GraphicsPlayfieldRenderer::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&playfield.get_view(), None, &[], true, &mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.drawn_blocks.is_empty()); // No tetromino blocks
        assert!(!display.drawn_rectangles.is_empty()); // But border is drawn
    }

    #[test]
    fn draw_renders_current_tetromino_blocks() {
        // Arrange
        let playfield = create_test_playfield();
        let sut = GraphicsPlayfieldRenderer::new();
        let tetromino = create_tetromino_instance(TetrominoType::O);
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(
            &playfield.get_view(),
            Some(&tetromino),
            &[],
            true,
            &mut display,
        );

        // Assert
        assert!(result.is_ok());
        assert!(!display.drawn_blocks.is_empty());

        // Verify all drawn blocks are O-type
        for (_, tetromino_type) in &display.drawn_blocks {
            assert_eq!(*tetromino_type, TetrominoType::O);
        }
    }

    #[test]
    fn draw_renders_placed_tetromino_blocks() {
        // Arrange
        let mut playfield = create_test_playfield();
        let tetromino = create_tetromino_instance(TetrominoType::O);
        playfield.set_current_tetromino(Some(tetromino));
        playfield.lock_tetromino();

        let sut = GraphicsPlayfieldRenderer::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&playfield.get_view(), None, &[], true, &mut display);

        // Assert
        assert!(result.is_ok());
        assert!(!display.drawn_blocks.is_empty());
    }

    #[test]
    fn draw_draws_nothing_on_empty_playfield_with_no_tetromino() {
        // Arrange
        let playfield = create_test_playfield();
        let sut = GraphicsPlayfieldRenderer::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&playfield.get_view(), None, &[], true, &mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.drawn_blocks.is_empty());
        assert!(!display.drawn_rectangles.is_empty()); // Border is still drawn
    }

    #[test]
    fn draw_border_draws_three_rectangles() {
        // Arrange
        let sut = GraphicsPlayfieldRenderer::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw_border(&mut display);

        // Assert
        assert!(result.is_ok());
        assert_eq!(display.drawn_rectangles.len(), 3); // Left, bottom, right borders

        // Verify all borders use white color
        for (_, _, _, _, color) in &display.drawn_rectangles {
            assert_eq!(*color, Color::WHITE);
        }
    }

    #[test]
    fn draw_with_hidden_blinking_lines_does_not_draw_them() {
        // Arrange
        let mut playfield = create_test_playfield();
        let tetromino = create_tetromino_instance(TetrominoType::O);
        playfield.set_current_tetromino(Some(tetromino));
        playfield.lock_tetromino();
        let sut = GraphicsPlayfieldRenderer::new();
        let mut display = MockDisplay::new();

        // Act
        let result = sut.draw(&playfield.get_view(), None, &[5, 6], false, &mut display);

        // Assert
        assert!(result.is_ok());
        assert!(display.drawn_blocks.is_empty());
    }

    fn create_test_playfield() -> Playfield<FixedTetrominoGenerator> {
        let dimensions = Dimensions::new(10, 20);
        Playfield::new(dimensions, FixedTetrominoGenerator::new(TetrominoType::O))
    }

    fn create_tetromino_instance(tetromino_type: TetrominoType) -> TetrominoInstance {
        let tetromino_definitions = TetrominoDefinitions::new();
        let position = Position::new(4, 4);
        TetrominoInstance::new(tetromino_type, position, &tetromino_definitions)
    }
}
