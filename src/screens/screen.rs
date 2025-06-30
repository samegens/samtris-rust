use crate::graphics::Display;
use crate::input::InputEvent;
use crate::screens::ScreenResult;
use std::time::Duration;

pub trait Screen {
    fn update(&mut self, delta_time: Duration);
    fn draw(&mut self, display: &mut dyn Display) -> Result<(), String>;
    fn handle_input(&mut self, input_events: &[InputEvent]) -> ScreenResult;
}
