use crate::render::Material;
use alexandria::math::{Color4f, Srgb};

impl Material {
    /// Set the color of the material
    pub(crate) fn set_color(&mut self, color: Color4f<Srgb>) {
        self.color = color;
    }

    /// Set the specular strength of the material
    pub(crate) fn set_specular_strength(&mut self, specular_strength: f32) {
        self.specular_strength = specular_strength;
    }

    /// Set the shininess of the material
    pub(crate) fn set_shininess(&mut self, shininess: f32) {
        self.shininess = shininess;
    }
}
