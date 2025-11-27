use crate::math::{Matrix4x4f, Transform};

impl Transform {
    /// Updates the transform if needed, returning if it was updated
    pub(crate) fn update(&mut self) -> bool {
        if !self.dirty {
            return false;
        }

        self.matrix = Matrix4x4f::scale(self.scale)
            * Matrix4x4f::rotation(self.rotation)
            * Matrix4x4f::translation(self.position);

        self.dirty = false;
        true
    }

    /// Updates the transform if needed, returning if it was updated
    ///
    /// This treats the transform as a Camera transform
    pub(crate) fn update_camera(&mut self) -> bool {
        if !self.dirty {
            return false;
        }

        self.matrix = Matrix4x4f::translation(-self.position)
            * Matrix4x4f::rotation(self.rotation.conjugate());

        self.dirty = false;
        true
    }
}
