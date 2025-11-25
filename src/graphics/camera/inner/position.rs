use crate::{graphics::CameraInner, math::Vector3f};

impl CameraInner {
    /// Get the position of the camera
    pub fn position(&self) -> Vector3f {
        self.transform.position()
    }

    /// Set the position of the camera
    pub fn set_position(&mut self, position: Vector3f) {
        self.transform.set_position(position);
    }
}
