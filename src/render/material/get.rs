use crate::render::Material;
use alexandria::math::{Color4f, Linear};

impl Material {
    /// Get the color of the material
    pub fn color(&self) -> Color4f<Linear> {
        self.color
    }

    /// Get the red component of the color of the material
    pub fn red(&self) -> f32 {
        self.color.r
    }

    /// Get the green component of the color of the material
    pub fn green(&self) -> f32 {
        self.color.g
    }

    /// Get the blue component of the color of the material
    pub fn blue(&self) -> f32 {
        self.color.b
    }
}
