use crate::render::LightingData;
use alexandria::math::{Color4f, Linear};

impl LightingData {
    /// Set the ambient light color for the frame
    pub fn set_ambient_light(&mut self, ambient_light: Color4f<Linear>) {
        self.metadata[0].ambient_light = ambient_light;
    }
}
