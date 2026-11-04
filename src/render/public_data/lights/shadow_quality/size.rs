use crate::render::ShadowQuality;

impl ShadowQuality {
    /// Get the directional light shadow map size for this quality
    pub fn directional_light_size(&self) -> f32 {
        match self {
            ShadowQuality::Low => 512.0,
            ShadowQuality::Medium => 1024.0,
            ShadowQuality::High => 2048.0,
        }
    }
}
