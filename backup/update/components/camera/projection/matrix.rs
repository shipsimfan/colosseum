use crate::update::components::CameraProjection;
use alexandria::math::Matrix4x4f;
use std::f32::consts::PI;

impl CameraProjection {
    /// Calculate the projection matrix for the camera projection
    pub(in crate::update::components::camera) fn matrix(&self, aspect: f32) -> Matrix4x4f {
        match self {
            &CameraProjection::Perspective { fov_y, near, far } => {
                Matrix4x4f::new_perspective(aspect, fov_y * PI / 180.0, near, far)
            }
            &CameraProjection::InfinitePerspective { fov_y, near, .. } => {
                Matrix4x4f::new_infinite_perspective(aspect, fov_y * PI / 180.0, near)
            }
            &CameraProjection::Orthographic { size, near, far } => {
                let x_size = size * aspect;
                Matrix4x4f::new_orthographic(-x_size, x_size, size, -size, near, far)
            }
        }
    }
}
