use crate::graphics::{Display, PlayfieldView};

pub trait PlayfieldRenderer {
    fn draw<D: Display>(
        &self,
        playfield_view: &PlayfieldView,
        display: &mut D,
    ) -> Result<(), String>;
}
