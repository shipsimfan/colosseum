use crate::{graphics::CameraInner, math::Vector3f};

impl CameraInner {
    /// Get the rotation of the camera
    pub fn rotation(&self) -> Vector3f {
        -self.transform.rotation()
    }

    /// Set the rotation of the camera
    pub fn set_rotation(&mut self, rotation: Vector3f) {
        self.transform.set_rotation(-rotation);
    }
}
