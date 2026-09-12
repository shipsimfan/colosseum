use crate::update::components::DirectionalLight;
use alexandria::math::{Color3f, Srgb, Vector3f};

impl DirectionalLight {
    /// Set the color of the directional light
    pub fn set_color<C: Into<Color3f<Srgb>>>(&mut self, color: C) {
        self.color = color.into().into_linear();
    }

    /// Set the intensity of the directional light
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    /// Set the direction of the light
    pub fn set_direction<D: Into<Vector3f>>(&mut self, direction: D) {
        self.direction = direction.into().normalized();
    }

    /// Set the depth to render shadows outside of the camera's frustum in the direction of the light
    pub fn set_shadow_depth(&mut self, shadow_depth: f32) {
        self.shadow_depth = shadow_depth;
    }
}
