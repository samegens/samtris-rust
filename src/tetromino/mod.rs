#[cfg(test)]
mod fixed_tetromino_generator;
mod random_tetromino_generator;
mod tetromino_definition;
mod tetromino_definitions;
mod tetromino_generator;
mod tetromino_instance;
mod tetromino_type;

#[cfg(test)]
pub use fixed_tetromino_generator::FixedTetrominoGenerator;

pub use random_tetromino_generator::RandomTetrominoGenerator;
pub use tetromino_definitions::TetrominoDefinitions;
pub use tetromino_generator::*;
pub use tetromino_instance::TetrominoInstance;
pub use tetromino_type::TetrominoType;
