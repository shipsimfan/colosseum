use crate::update::components::CameraProjection;
use alexandria::math::{Matrix4x4f, Vector2u};
use std::f32::consts::PI;

impl CameraProjection {
    /// Calculate the projection matrix for the camera projection
    pub(in crate::update::components::camera) fn matrix(
        &self,
        window_size: Vector2u,
    ) -> Matrix4x4f {
        match self {
            &CameraProjection::Perspective { fov_y, near, far } => {
                let aspect = window_size.x as f32 / window_size.y as f32;
                Matrix4x4f::new_perspective(aspect, fov_y * PI / 180.0, near, far)
            }
            &CameraProjection::InfinitePerspective { fov_y, near } => {
                let aspect = window_size.x as f32 / window_size.y as f32;
                Matrix4x4f::new_infinite_perspective(aspect, fov_y * PI / 180.0, near)
            }
            &CameraProjection::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => Matrix4x4f::new_orthographic(left, right, top, bottom, near, far),
        }
    }
}
