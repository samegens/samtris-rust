// src/graphics/mod.rs
mod color;
mod display;
mod graphics_hud_renderer;
mod graphics_playfield_renderer;
mod hud_renderer;
mod hud_view;
#[cfg(test)]
mod mock_display;
#[cfg(test)]
mod mock_hud_renderer;
#[cfg(test)]
mod mock_playfield_renderer;
mod playfield_renderer;
mod playfield_view;
mod sdl_display;
mod tetromino_pattern;

pub use color::Color;
pub use display::Display;
pub use graphics_hud_renderer::GraphicsHudRenderer;
pub use graphics_playfield_renderer::GraphicsPlayfieldRenderer;
pub use hud_renderer::HudRenderer;
pub use hud_view::HudView;
#[cfg(test)]
pub use mock_display::MockDisplay;
#[cfg(test)]
pub use mock_hud_renderer::MockHudRenderer;
#[cfg(test)]
pub use mock_playfield_renderer::MockPlayfieldRenderer;
pub use playfield_renderer::PlayfieldRenderer;
pub use playfield_view::PlayfieldView;
pub use sdl_display::SdlDisplay;
pub use tetromino_pattern::TetrominoPattern;
