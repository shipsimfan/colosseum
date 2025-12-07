use crate::{graphics::AmbientLight, math::Color3f};

impl AmbientLight {
    /// Get the ambient light color
    pub fn color(&self) -> Color3f {
        self.constant_buffer.ambient_color
    }

    /// Get the intensity of the ambient light
    pub fn intensity(&self) -> f32 {
        self.constant_buffer.ambient_intensity
    }
}
