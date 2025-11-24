use crate::math::{Quaternionf, Transform};

impl Transform {
    /// Get the rotation
    pub const fn rotation(&self) -> Quaternionf {
        self.rotation
    }

    /// Set the rotation
    pub fn set_rotation(&mut self, rotation: Quaternionf) {
        self.rotation = rotation;
        self.dirty = true;
    }
}
