use crate::graphics::{Display, PlayfieldView};

pub trait PlayfieldRenderer {
    fn draw<D: Display>(
        &self,
        playfield_view: &PlayfieldView,
        blinking_lines: &[u32],
        show_blinking_lines: bool,
        display: &mut D,
    ) -> Result<(), String>;
}
