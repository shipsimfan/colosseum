use crate::math::{Transform, Vector3f};

impl Transform {
    /// Get the rotation
    pub const fn rotation(&self) -> Vector3f {
        self.rotation
    }

    /// Set the rotation
    pub fn set_rotation(&mut self, rotation: Vector3f) {
        self.rotation = rotation;
        self.dirty = true;
    }
}
