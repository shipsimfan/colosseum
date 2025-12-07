use crate::{
    Transform,
    math::{Matrix4x4f, Quaternionf, Vector3f},
};

impl Transform {
    /// Get the position
    pub const fn position(&self) -> Vector3f {
        self.position
    }

    /// Get the rotation
    pub const fn rotation(&self) -> Quaternionf {
        self.rotation
    }

    /// Get the scale
    pub const fn scale(&self) -> Vector3f {
        self.scale
    }

    /// Get the matrix that represents this transform
    pub(crate) const fn matrix(&self) -> Matrix4x4f {
        self.matrix
    }
}
