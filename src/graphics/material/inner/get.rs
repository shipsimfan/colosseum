use crate::{graphics::MaterialInner, math::Color3f};

impl MaterialInner {
    /// Gets the color assigned to this material
    pub fn color(&self) -> Color3f {
        self.buffer.color
    }

    /// Gets the strength of the specular highlight
    pub fn specular_strength(&self) -> f32 {
        self.buffer.specular_strength
    }
}
