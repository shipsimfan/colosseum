use crate::math::{Matrix4x4f, Transform, Vector3f};

impl Transform {
    /// Creates a new default [`Transform`]
    pub const fn new() -> Self {
        Transform {
            position: Vector3f::ZERO,
            rotation: Vector3f::ZERO,
            scale: Vector3f::ONE,
            matrix: Matrix4x4f::identity(),
            dirty: false,
        }
    }
}
