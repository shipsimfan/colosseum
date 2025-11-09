use crate::{
    graphics::CameraProjection,
    math::{Matrix4x4f, Vector2u},
};

impl CameraProjection {
    /// Produce the matrix for this projection
    pub(in crate::graphics::camera) fn matrix(&self, screen_size: Vector2u) -> Matrix4x4f {
        let aspect = screen_size.x as f32 / screen_size.y as f32;
        match self {
            CameraProjection::Perspective { fov, near, far } => {
                Matrix4x4f::perspective(aspect, *fov, *near, *far)
            }
            CameraProjection::Orthographic { size, near, far } => {
                let left = *size * aspect;
                Matrix4x4f::orthographic(left, -left, *size, -*size, *near, *far)
            }
        }
    }
}
