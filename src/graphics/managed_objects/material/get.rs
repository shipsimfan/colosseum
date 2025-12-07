use crate::{graphics::Material, math::Color3f};

impl Material {
    /// Gets the color assigned to this material
    pub fn color(&self) -> Color3f {
        self.buffer.color
    }

    /// Gets the strength of the specular highlight
    pub fn specular_strength(&self) -> f32 {
        self.buffer.specular_strength
    }

    /// Get the ID assigned to this material
    pub(in crate::graphics) fn id(&self) -> u32 {
        self.id.get()
    }
}
