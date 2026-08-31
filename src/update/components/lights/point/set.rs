use crate::update::components::PointLight;
use alexandria::math::{Color3f, Srgb, Vector3f};

impl PointLight {
    /// Set the color of the point light
    pub fn set_color<C: Into<Color3f<Srgb>>>(&mut self, color: C) {
        self.color = color.into().into_linear();
    }

    /// Set the intensity of the point light
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    /// Set the direction of the light
    pub fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        self.position = position.into();
    }

    /// Set the range of the light
    pub fn set_range(&mut self, range: f32) {
        self.range = range;
    }
}
