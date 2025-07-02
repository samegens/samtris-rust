mod game_screen;
mod high_scores_screen;
mod menu_screen;
mod screen;
mod screen_result;

pub use game_screen::GameScreen;
//TODO: remove this when screen is used in game
#[allow(unused_imports)]
pub use high_scores_screen::HighScoresScreen;
pub use menu_screen::MenuScreen;
pub use screen::Screen;
pub use screen_result::ScreenResult;
