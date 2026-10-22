use crate::render::ColorCorrection;

impl ColorCorrection {
    /// Create a new default set of [`ColorCorrection`] settings
    pub fn new() -> ColorCorrection {
        ColorCorrection {
            contrast: 1.0,
            saturation: 1.0,
            exposure: 1.0,
        }
    }
}
