use crate::math::{Matrix4x4f, Transform};

impl Transform {
    /// Get the matrix that represents this transform
    pub(crate) const fn matrix(&self) -> Matrix4x4f {
        self.matrix
    }
}
