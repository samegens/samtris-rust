use crate::graphics::{Display, HudRenderer, HudView};

#[derive(Debug, Default)]
pub struct MockHudRenderer {
    pub draw_calls: std::cell::RefCell<Vec<HudView>>,
}

impl MockHudRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_draw_calls(&self) -> Vec<HudView> {
        self.draw_calls.borrow().clone()
    }
}

impl HudRenderer for MockHudRenderer {
    fn draw<D: Display>(&self, hud_view: &HudView, _display: &mut D) -> Result<(), String> {
        self.draw_calls.borrow_mut().push(hud_view.clone());
        Ok(())
    }
}
