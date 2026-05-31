use crate::render::RenderData;
use alexandria::math::{Color3f, Linear};

impl RenderData {
    /// Get the color to clear the screen to before rendering
    pub fn clear_color(&self) -> Color3f<Linear> {
        self.clear_color
    }
}
