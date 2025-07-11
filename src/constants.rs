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
pub const FILLED_LINES_ANIMATION_DURATION_MS: u64 = 1000;
pub const NEXT_TETROMINO_OFFSET_X: u32 =
    PLAYFIELD_OFFSET_X + (PLAYFIELD_WIDTH * BLOCK_SIZE) + (2 * BLOCK_SIZE);
pub const NEXT_TETROMINO_OFFSET_Y: u32 = PLAYFIELD_OFFSET_Y + (2 * BLOCK_SIZE);
pub const NEXT_TETROMINO_AREA_WIDTH: u32 = 5 * BLOCK_SIZE;
pub const NEXT_TETROMINO_AREA_HEIGHT: u32 = 4 * BLOCK_SIZE;
pub const SCORE_OFFSET_X: u32 = 20;
pub const SCORE_OFFSET_Y: u32 = NEXT_TETROMINO_OFFSET_Y;
pub const LINES_OFFSET_X: u32 = 20;
pub const LINES_OFFSET_Y: u32 = NEXT_TETROMINO_OFFSET_Y + 20;
pub const LEVEL_OFFSET_X: u32 = 20;
pub const LEVEL_OFFSET_Y: u32 = NEXT_TETROMINO_OFFSET_Y + 40;
pub const GAME_OVER_WIDTH: u32 = (PLAYFIELD_WIDTH + 2) * BLOCK_SIZE;
pub const GAME_OVER_HEIGHT: u32 = 50;
pub const CHAR_WIDTH: u32 = 16;
pub const CHAR_HEIGHT: u32 = 16;
pub const HIGH_SCORES_FILE: &str = "high_scores.dat";
pub const HIGH_SCORES_X: u32 = 90;
pub const HIGH_SCORES_Y: u32 = 70;
pub const HIGH_SCORES_LINE_HEIGHT: u32 = 22;
