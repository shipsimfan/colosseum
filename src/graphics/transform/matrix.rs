use crate::{graphics::Transform, math::Matrix4x4f};

impl Transform {
    /// Get the matrix that represents this transform
    pub(in crate::graphics) fn matrix(&self) -> Matrix4x4f {
        self.matrix
    }
}
