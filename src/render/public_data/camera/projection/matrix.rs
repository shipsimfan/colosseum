use crate::render::CameraProjection;
use alexandria::math::Matrix4x4f;
use std::f32::consts::PI;

impl CameraProjection {
    /// Calculate the projection matrix for the camera projection
    pub(in crate::render::public_data::camera) fn matrix(&self, aspect: f32) -> Matrix4x4f {
        match self {
            &CameraProjection::Perspective { fov_y, near, far } => {
                let mut m = Matrix4x4f::new_perspective(aspect, fov_y * PI / 180.0, near, far);
                m[2][2] = -m[2][2];
                m[2][3] = -m[2][3];
                m
            }
            &CameraProjection::InfinitePerspective { fov_y, near, .. } => {
                let mut m = Matrix4x4f::new_infinite_perspective(aspect, fov_y * PI / 180.0, near);
                m[2][2] = 0.0;
                m[2][3] = -m[2][3];
                m
            }
            &CameraProjection::Orthographic { size, near, far } => {
                let x_size = size * aspect;
                let mut m = Matrix4x4f::new_orthographic(-x_size, x_size, size, -size, near, far);
                m[2][2] = -m[2][2];
                m[2][3] = -m[2][3];
                m
            }
        }
    }
}
