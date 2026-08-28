use crate::render::Material;
use alexandria::math::{Color4f, Srgb};

impl Material {
    /// Set the color of the material
    pub(crate) fn set_color(&mut self, color: Color4f<Srgb>) {
        self.color = color;
    }
}
