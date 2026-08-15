use crate::update::components::Transform;
use alexandria::math::{Matrix4x4f, Quaternionf, Vector3f};

impl Transform {
    /// Create a new [`Transform`]
    pub fn new(position: Vector3f, rotation: Quaternionf, scale: Vector3f) -> Self {
        Transform {
            position,
            rotation,
            scale,
            dirty: true,
            camera: false,
            matrix: Matrix4x4f::IDENTITY,
        }
    }
}
