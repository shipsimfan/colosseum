use crate::math::{Matrix4x4f, Quaternionf, Transform, Vector3f};

impl Transform {
    /// Creates a new default [`Transform`]
    pub const fn new() -> Self {
        Transform {
            position: Vector3f::ZERO,
            rotation: Quaternionf::IDENTITY,
            scale: Vector3f::ONE,
            matrix: Matrix4x4f::IDENTITY,
            dirty: false,
        }
    }
}
