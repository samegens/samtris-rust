use crate::common::{Dimensions, Position};
use crate::tetromino::{TetrominoInstance, TetrominoType};

pub struct PlayfieldView<'a> {
    pub dimensions: Dimensions,
    pub grid: &'a Vec<Vec<Option<TetrominoType>>>,
    pub current_tetromino: Option<&'a TetrominoInstance>,
}

impl<'a> PlayfieldView<'a> {
    pub fn is_position_occupied(&self, position: Position) -> bool {
        if !self.dimensions.contains(position) {
            return false;
        }

        let x = position.x as u32;
        let y = position.y as u32;

        self.is_xy_occupied(x, y)
    }

    fn is_xy_occupied(&self, x: u32, y: u32) -> bool {
        self.grid[y as usize][x as usize].is_some()
    }

    pub fn get_tetromino_type_at(&self, position: Position) -> Option<TetrominoType> {
        if !self.dimensions.contains(position) {
            return None;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        self.grid[y][x]
    }
}
