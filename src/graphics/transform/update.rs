use crate::{graphics::Transform, math::Matrix4x4f};

impl Transform {
    /// Updates the transform if needed, returning if it was updated
    pub(in crate::graphics) fn update(&mut self) -> bool {
        if !self.dirty {
            return false;
        }

        self.matrix = Matrix4x4f::translation(self.position)
            * Matrix4x4f::euler_rotation(self.rotation)
            * Matrix4x4f::scale(self.scale);

        self.dirty = false;
        true
    }
}
