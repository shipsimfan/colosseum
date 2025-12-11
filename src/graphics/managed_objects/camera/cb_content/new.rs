use crate::{
    graphics::managed_objects::camera::CameraCbContent,
    math::{Matrix4x4f, Vector2f, Vector3f},
};

impl CameraCbContent {
    /// Create a new [`CameraCbContent`]
    pub fn new(view: Matrix4x4f) -> CameraCbContent {
        CameraCbContent {
            view,
            position: Vector3f::ZERO,
            render_scale: 0.0,
            render_size: Vector2f::ZERO,
            inverse_render_size: Vector2f::ZERO,
        }
    }
}
