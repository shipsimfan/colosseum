use crate::graphics::{Camera, CameraProjection};

impl Camera {
    /// Get the current projection being used
    pub fn projection(&self) -> CameraProjection {
        self.projection
    }

    /// Set the projection being used by the camera
    pub fn set_projection(&mut self, projection: CameraProjection) {
        self.projection = projection;
        self.projection_dirty = true;
    }
}
