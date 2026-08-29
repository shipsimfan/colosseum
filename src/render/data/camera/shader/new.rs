use crate::render::data::camera::CameraShaderData;
use alexandria::math::{Matrix4x4f, Vector3f};

impl CameraShaderData {
    /// Create a new [`CameraShaderData`]
    pub fn new() -> CameraShaderData {
        CameraShaderData {
            view_projection: Matrix4x4f::IDENTITY,
            position: Vector3f::ZERO,
        }
    }
}
