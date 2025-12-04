use crate::{graphics::context::Lights, math::Color3f};

impl Lights {
    /// Get the ambient light color
    pub fn ambient_color(&self) -> Color3f {
        self.constant_buffer.ambient_color
    }

    /// Get the intensity of the ambient light
    pub fn ambient_intensity(&self) -> f32 {
        self.constant_buffer.ambient_intensity
    }
}
