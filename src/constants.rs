pub const BLOCK_SIZE: u32 = 16; // Length of block in pixels (also height)
pub const PLAYFIELD_WIDTH: u32 = 10; // In blocks
pub const PLAYFIELD_HEIGHT: u32 = 20; // In blocks
pub const TETRIS_SPAWN_X: i32 = 3; // In blocks within playfield, x = 0 is left
pub const TETRIS_SPAWN_Y: i32 = 0; // In blocks within playfield, y = 0 is up
pub const WINDOW_WIDTH_IN_BLOCKS: u32 = 40;
pub const WINDOW_HEIGHT_IN_BLOCKS: u32 = 25;
pub const PLAYFIELD_OFFSET_X: u32 = (WINDOW_WIDTH_IN_BLOCKS - PLAYFIELD_WIDTH) * BLOCK_SIZE / 2;
pub const PLAYFIELD_OFFSET_Y: u32 = (WINDOW_HEIGHT_IN_BLOCKS - PLAYFIELD_HEIGHT) * BLOCK_SIZE / 2;
pub const PLAYFIELD_BORDER_WIDTH: u32 = 1;
