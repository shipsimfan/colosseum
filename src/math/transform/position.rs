use crate::math::{Transform, Vector3f};

impl Transform {
    /// Get the position
    pub const fn position(&self) -> Vector3f {
        self.position
    }

    /// Set the position
    pub fn set_position(&mut self, position: Vector3f) {
        self.position = position;
        self.dirty = true;
    }
}
