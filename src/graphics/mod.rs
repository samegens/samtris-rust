// src/graphics/mod.rs
pub mod color;
pub mod display;
pub mod mock_display;
pub mod sdl_display;

pub use color::Color;
pub use display::Display;
pub use sdl_display::SdlDisplay;
