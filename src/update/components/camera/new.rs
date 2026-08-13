use crate::update::components::{Camera, CameraProjection};
use alexandria::math::{Matrix4x4f, Vector2u};

impl Camera {
    /// Create a new [`Camera`]
    pub fn new(projection: CameraProjection) -> Camera {
        Camera {
            projection,
            last_viewport_size: Vector2u::new(0, 0),
            projection_dirty: true,
            projection_matrix: Matrix4x4f::IDENTITY,
        }
    }

    /// Create a new [`Camera`] with a perspective projection
    pub fn new_perspective(fov_y: f32, near: f32, far: f32) -> Camera {
        Camera::new(CameraProjection::Perspective { fov_y, near, far })
    }

    /// Create a new [`Camera`] with an infinite perspective projection
    pub fn new_infinite_perspective(fov_y: f32, near: f32) -> Camera {
        Camera::new(CameraProjection::InfinitePerspective { fov_y, near })
    }

    /// Create a new [`Camera`] with an orthographic projection
    pub fn new_orthographic(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        Camera::new(CameraProjection::Orthographic {
            left,
            right,
            bottom,
            top,
            near,
            far,
        })
    }
}
