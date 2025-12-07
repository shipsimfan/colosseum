use crate::{Transform, math::Matrix4x4f};

impl Transform {
    /// Updates the transform if needed, returning the current epoch
    pub(crate) fn update(&mut self) -> u32 {
        if !self.dirty {
            return self.epoch;
        }

        self.matrix = Matrix4x4f::scale(self.scale)
            * Matrix4x4f::rotation(self.rotation)
            * Matrix4x4f::translation(self.position);

        self.dirty = false;
        self.epoch += 1;
        self.epoch
    }

    /// Updates the transform if needed, returning the current epoch
    ///
    /// This treats the transform as a camera transform
    pub(crate) fn update_camera(&mut self) -> u32 {
        if !self.dirty {
            return self.epoch;
        }

        self.matrix = Matrix4x4f::translation(-self.position)
            * Matrix4x4f::rotation(self.rotation.conjugate());

        self.dirty = false;
        self.epoch += 1;
        self.epoch
    }
}
