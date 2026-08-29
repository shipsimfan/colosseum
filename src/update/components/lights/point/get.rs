use crate::update::components::PointLight;
use alexandria::math::{Color3f, Linear, Vector3f};

impl PointLight {
    /// Get the color of the point light
    pub fn color(&self) -> Color3f<Linear> {
        self.color
    }

    /// Get the intensity of the point light
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Get the position of the point light
    pub fn position(&self) -> Vector3f {
        self.position
    }

    /// Get the range of the point light
    pub fn range(&self) -> f32 {
        self.range
    }
}
