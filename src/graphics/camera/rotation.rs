use crate::{graphics::Camera, math::Quaternionf};

impl Camera {
    /// Get the rotation of the camera
    pub fn rotation(&self) -> Quaternionf {
        self.transform.rotation()
    }

    /// Set the rotation of the camera
    pub fn set_rotation(&mut self, rotation: Quaternionf) {
        self.transform.set_rotation(rotation);
    }
}
