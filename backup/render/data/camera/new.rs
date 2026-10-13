use crate::render::RenderCamera;
use alexandria::math::{Matrix4x4f, Vector3f};

impl RenderCamera {
    /// Create a new [`CameraRenderData`]
    pub fn new() -> RenderCamera {
        RenderCamera {
            view_projection: Matrix4x4f::IDENTITY,
            position: Vector3f::ZERO,
        }
    }
}
