use crate::render::{Camera, CameraProjection};
use alexandria::math::{Quaternionf, Vector3f};

impl Camera {
    /// Create a new [`Camera`]
    pub fn new(position: Vector3f, rotation: Quaternionf, projection: CameraProjection) -> Camera {
        Camera {
            position,
            rotation,
            projection,
        }
    }

    /// Create a new [`Camera`] with a perspective projection
    pub fn new_perspective(
        position: Vector3f,
        rotation: Quaternionf,
        fov_y: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        Camera::new(
            position,
            rotation,
            CameraProjection::Perspective { fov_y, near, far },
        )
    }

    /// Create a new [`Camera`] with an infinite perspective projection
    pub fn new_infinite_perspective(
        position: Vector3f,
        rotation: Quaternionf,
        fov_y: f32,
        near: f32,
        shadow_distance: f32,
    ) -> Camera {
        Camera::new(
            position,
            rotation,
            CameraProjection::InfinitePerspective {
                fov_y,
                near,
                shadow_distance,
            },
        )
    }

    /// Create a new [`Camera`] with an orthographic projection
    pub fn new_orthographic(
        position: Vector3f,
        rotation: Quaternionf,
        size: f32,
        near: f32,
        far: f32,
    ) -> Camera {
        Camera::new(
            position,
            rotation,
            CameraProjection::Orthographic { size, near, far },
        )
    }
}
