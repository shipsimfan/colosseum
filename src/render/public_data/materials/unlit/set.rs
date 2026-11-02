use crate::render::UnlitMaterial;
use alexandria::math::{Color4f, Srgb};

impl UnlitMaterial {
    /// Set the color of the material
    pub fn set_color<C: Into<Color4f<Srgb>>>(&mut self, color: C) {
        self.color = color.into().into_linear();
    }
}
