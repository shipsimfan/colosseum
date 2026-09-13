use crate::update::components::{Camera, CameraProjection};
use alexandria::math::{Matrix4x4f, Vector2u, Vector3f};

impl Camera {
    /// Create a new [`Camera`]
    pub fn new(projection: CameraProjection) -> Camera {
        Camera {
            projection,
            last_viewport_size: Vector2u::new(0, 0),
            projection_dirty: true,
            projection_matrix: Matrix4x4f::IDENTITY,
            corners: [([Vector3f::ZERO; 4], 0.0); 5],
        }
    }

    /// Create a new [`Camera`] with a perspective projection
    pub fn new_perspective(fov_y: f32, near: f32, far: f32) -> Camera {
        Camera::new(CameraProjection::Perspective { fov_y, near, far })
    }

    /// Create a new [`Camera`] with an infinite perspective projection
    pub fn new_infinite_perspective(fov_y: f32, near: f32, shadow_distance: f32) -> Camera {
        Camera::new(CameraProjection::InfinitePerspective {
            fov_y,
            near,
            shadow_distance,
        })
    }

    /// Create a new [`Camera`] with an orthographic projection
    pub fn new_orthographic(size: f32, near: f32, far: f32) -> Camera {
        Camera::new(CameraProjection::Orthographic { size, near, far })
    }
}
