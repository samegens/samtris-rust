use crate::graphics::{Display, HudView};

pub trait HudRenderer {
    fn draw<D: Display + ?Sized>(&self, hud_view: &HudView, display: &mut D) -> Result<(), String>;
}
