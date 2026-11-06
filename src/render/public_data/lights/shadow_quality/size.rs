use crate::render::ShadowQuality;

impl ShadowQuality {
    /// Get the directional light shadow map size for this quality
    pub fn directional_light_size(&self) -> u32 {
        match self {
            ShadowQuality::Low => 512,
            ShadowQuality::Medium => 1024,
            ShadowQuality::High => 2048,
        }
    }

    /// Get the point light shadow map size for this quality
    pub fn point_light_size(&self) -> u32 {
        match self {
            ShadowQuality::Low => 256,
            ShadowQuality::Medium => 512,
            ShadowQuality::High => 1024,
        }
    }

    /// Get the spot light shadow map size for this quality
    pub fn spot_light_size(&self) -> u32 {
        match self {
            ShadowQuality::Low => 512,
            ShadowQuality::Medium => 1024,
            ShadowQuality::High => 2048,
        }
    }
}
