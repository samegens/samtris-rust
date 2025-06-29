use crate::{graphics::Display, menu::Menu};

//TODO remove once use from main
#[allow(dead_code)]
pub trait MenuRenderer {
    fn draw<D: Display>(&self, menu: &Menu, display: &mut D) -> Result<(), String>;
}
