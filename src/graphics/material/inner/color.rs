use crate::{graphics::MaterialInner, math::Color3f};

impl MaterialInner {
    /// Gets the color assigned to this material
    pub fn color(&self) -> Color3f {
        self.color
    }

    /// Set the color used by the material
    pub fn set_color(&mut self, color: Color3f) {
        self.color = color;
        self.dirty = true;
    }
}
