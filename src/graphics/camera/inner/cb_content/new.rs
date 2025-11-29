use crate::{
    graphics::camera::inner::CameraCbContent,
    math::{Matrix4x4f, Vector3f},
};

impl CameraCbContent {
    /// Create a new [`CameraCbContent`]
    pub fn new(view: Matrix4x4f) -> CameraCbContent {
        CameraCbContent {
            view,
            position: Vector3f::ZERO,
            reserved: 0.0,
        }
    }
}
