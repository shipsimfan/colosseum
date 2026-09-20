use crate::render::{LightingData, ShadowQuality};
use alexandria::math::{Color4f, Linear};

impl LightingData {
    /// Set the ambient light color for the frame
    pub fn set_ambient_light(&mut self, ambient_light: Color4f<Linear>) {
        self.metadata[0].ambient_light = ambient_light;
    }

    /// Set the shadow quality for the frame
    pub fn set_shadow_quality(&mut self, shadow_quality: ShadowQuality) {
        self.shadow_quality = shadow_quality;
    }
}
