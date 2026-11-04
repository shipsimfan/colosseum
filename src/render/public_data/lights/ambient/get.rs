use crate::render::AmbientLight;
use alexandria::math::{Color3f, Linear};

impl AmbientLight {
    /// Get the color of the ambient light
    pub fn color(&self) -> Color3f<Linear> {
        self.color
    }
}
