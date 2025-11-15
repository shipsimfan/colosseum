use crate::math::{Transform, Vector3f};

impl Transform {
    /// Get the scale
    pub const fn scale(&self) -> Vector3f {
        self.scale
    }

    /// Set the scale
    pub fn set_scale(&mut self, scale: Vector3f) {
        self.scale = scale;
        self.dirty = true;
    }
}
