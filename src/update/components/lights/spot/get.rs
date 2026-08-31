use crate::update::components::SpotLight;
use alexandria::math::{Color3f, Linear, Vector3f};

impl SpotLight {
    /// Get the color of the spot light
    pub fn color(&self) -> Color3f<Linear> {
        self.color
    }

    /// Get the intensity of the spot light
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Get the position of the spot light
    pub fn position(&self) -> Vector3f {
        self.position
    }

    /// Get the range of the spot light
    pub fn range(&self) -> f32 {
        self.range
    }

    /// Get the direction of the spot light
    pub fn direction(&self) -> Vector3f {
        self.direction
    }

    /// Get the cut off angle of the spot light
    pub fn cutoff_angle(&self) -> f32 {
        self.cutoff_angle.acos()
    }

    /// Get angle at which light begins falling off
    pub fn falloff_angle(&self) -> f32 {
        self.falloff_angle.acos()
    }
}
