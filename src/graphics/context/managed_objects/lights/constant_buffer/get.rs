use crate::{graphics::context::managed_objects::lights::LightConstantBuffer, math::Color3f};

impl LightConstantBuffer {
    /// Get the ambient light color
    pub fn ambient_color(&self) -> Color3f {
        self.content.ambient_color
    }

    /// Get the intensity of the ambient light
    pub fn ambient_intensity(&self) -> f32 {
        self.content.ambient_intensity
    }
}
