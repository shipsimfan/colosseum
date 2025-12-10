use crate::graphics::{Camera, CameraPostProcessing};

impl Camera {
    /// Get the post-processing effects applied to this camera
    pub fn post_processing(&mut self) -> &mut CameraPostProcessing {
        &mut self.post_processing
    }
}
