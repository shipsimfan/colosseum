use crate::{graphics::MaterialInner, math::Color3f};

impl MaterialInner {
    /// Set the color used by the material
    pub fn set_color(&mut self, color: Color3f) {
        self.buffer_content.color = color;
        self.dirty = true;
    }

    /// Set the strength of the specular highlight
    pub fn set_specular_strength(&mut self, specular_strength: f32) {
        self.buffer_content.specular_strength = specular_strength;
        self.dirty = true;
    }
}
