use crate::{
    Transform,
    math::{Quaternionf, Vector3f},
};

impl Transform {
    /// Set the position
    pub fn set_position(&mut self, position: Vector3f) {
        self.position = position;
        self.dirty = true;
    }

    /// Set the rotation
    pub fn set_rotation(&mut self, rotation: Quaternionf) {
        self.rotation = rotation;
        self.dirty = true;
    }

    /// Set the scale
    pub fn set_scale(&mut self, scale: Vector3f) {
        self.scale = scale;
        self.dirty = true;
    }
}
