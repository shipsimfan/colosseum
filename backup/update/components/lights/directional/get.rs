use crate::update::components::DirectionalLight;
use alexandria::math::{Color3f, Linear, Vector3f};

impl DirectionalLight {
    /// Get the color of the directional light
    pub fn color(&self) -> Color3f<Linear> {
        self.color
    }

    /// Get the intensity of the directional light
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Get the direction of the light
    pub fn direction(&self) -> Vector3f {
        self.direction
    }

    /// Get the depth to render shadows outside of the camera's frustum in the direction of the light
    pub fn shadow_depth(&self) -> f32 {
        self.shadow_depth
    }
}
