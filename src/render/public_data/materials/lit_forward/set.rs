use crate::render::LitForwardMaterial;
use alexandria::math::{Color3f, Srgb};

impl LitForwardMaterial {
    /// Set the color of the material
    pub fn set_color<C: Into<Color3f<Srgb>>>(&mut self, color: C) {
        self.color = color.into().into_linear();
    }
}
