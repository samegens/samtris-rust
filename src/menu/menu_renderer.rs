use crate::{graphics::Display, menu::Menu};

pub trait MenuRenderer {
    fn draw<D: Display + ?Sized>(&self, menu: &Menu, display: &mut D) -> Result<(), String>;
}
