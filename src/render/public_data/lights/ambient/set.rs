use crate::render::AmbientLight;
use alexandria::math::{Color3f, Linear};

impl AmbientLight {
    /// Set the color of the ambient light
    pub fn set_color<C: Into<Color3f<Linear>>>(&mut self, color: C) {
        self.color = color.into();
    }
}
