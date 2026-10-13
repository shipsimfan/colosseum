use crate::render::{PerFrameObjectBuilder, ShadowQuality};

impl<'a> PerFrameObjectBuilder<'a> {
    /// Get the shadow quality for the frame
    pub fn shadow_quality(&self) -> ShadowQuality {
        self.shadow_quality
    }
}
