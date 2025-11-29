use crate::{graphics::context::managed_objects::lights::LightConstantBuffer, math::Color3f};

impl LightConstantBuffer {
    /// Set the ambient light color
    pub fn set_ambient_color(&mut self, ambient_color: Color3f) {
        self.content.ambient_color = ambient_color;
        self.dirty = true;
    }

    /// Set the intensity of the ambient light
    pub fn set_ambient_intensity(&mut self, ambient_intensity: f32) {
        self.content.ambient_intensity = ambient_intensity;
        self.dirty = true;
    }
}
