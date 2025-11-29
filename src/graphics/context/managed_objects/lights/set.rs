use crate::{graphics::context::Lights, math::Color3f};

impl Lights {
    /// Set the ambient light color
    pub fn set_ambient_color(&mut self, ambient_color: Color3f) {
        self.constant_buffer.set_ambient_color(ambient_color);
    }

    /// Set the intensity of the ambient light
    pub fn set_ambient_intensity(&mut self, ambient_intensity: f32) {
        self.constant_buffer
            .set_ambient_intensity(ambient_intensity);
    }
}
