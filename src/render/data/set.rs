use crate::render::RenderData;
use alexandria::math::{Color3f, Linear};

impl RenderData {
    /// Set the color to clear the screen to before rendering
    pub fn set_clear_color(&mut self, clear_color: Color3f<Linear>) {
        self.clear_color = clear_color;
    }
}
