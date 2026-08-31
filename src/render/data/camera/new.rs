use crate::render::data::CameraRenderData;
use alexandria::math::{Matrix4x4f, Vector3f};

impl CameraRenderData {
    /// Create a new [`CameraRenderData`]
    pub fn new() -> CameraRenderData {
        CameraRenderData {
            view_projection: Matrix4x4f::IDENTITY,
            position: Vector3f::ZERO,
        }
    }
}
