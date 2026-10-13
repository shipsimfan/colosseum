use crate::render::Material;
use alexandria::math::{Color4f, Srgb};

impl Material {
    /// Get the color of the material
    pub fn color(&self) -> Color4f<Srgb> {
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

    /// Get the specular strength of the material
    pub fn specular_strength(&self) -> f32 {
        self.specular_strength
    }

    /// Get the shininess of the material
    pub fn shininess(&self) -> f32 {
        self.shininess
    }
}
