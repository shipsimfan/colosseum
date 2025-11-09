use crate::{
    graphics::Transform,
    math::{Matrix4x4f, Vector3f},
};

impl Transform {
    /// Creates a new default [`Transform`]
    pub fn new() -> Self {
        Transform {
            position: Vector3f::ZERO,
            rotation: Vector3f::ZERO,
            scale: Vector3f::ONE,
            matrix: Matrix4x4f::identity(),
            dirty: false,
        }
    }
}
