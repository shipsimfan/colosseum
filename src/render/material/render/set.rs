use crate::render::RenderMaterial;
use alexandria::math::{Color4f, Linear};

impl RenderMaterial {
    /// Set the color of the material
    pub fn set_color(&mut self, color: Color4f<Linear>) {
        self.color = color;
    }
}
