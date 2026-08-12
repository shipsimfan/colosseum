use crate::render::{CameraRenderData, data::DoubledRenderData};

impl DoubledRenderData {
    /// Get the camera data for the current frame
    pub fn camera(&self) -> &CameraRenderData {
        &self.camera
    }
}
