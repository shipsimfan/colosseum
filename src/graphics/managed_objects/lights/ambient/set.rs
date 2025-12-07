use crate::{graphics::AmbientLight, math::Color3f};

impl AmbientLight {
    /// Set the ambient light color
    pub fn set_color(&mut self, ambient_color: Color3f) {
        self.constant_buffer.ambient_color = ambient_color;
    }

    /// Set the intensity of the ambient light
    pub fn set_intensity(&mut self, ambient_intensity: f32) {
        self.constant_buffer.ambient_intensity = ambient_intensity;
    }
}
