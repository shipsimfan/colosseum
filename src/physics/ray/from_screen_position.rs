use crate::{
    physics::Ray,
    render::{Camera, CameraProjection},
};
use alexandria::math::{Vector2f, Vector2i, Vector2u};

impl Ray {
    /// Create a new [`Ray`] from a screen position
    pub fn from_screen_position(
        screen_position: Vector2i,
        screen_size: Vector2u,
        camera: &Camera,
    ) -> Ray {
        let normalized_screen_position = screen_position.into_f32() / screen_size.into_f32();
        let aspect = screen_size.x as f32 / screen_size.y as f32;

        match camera.projection {
            CameraProjection::InfinitePerspective { fov_y, .. }
            | CameraProjection::Perspective { fov_y, .. } => {
                Ray::from_perspective(normalized_screen_position, aspect, fov_y, camera)
            }
            CameraProjection::Orthographic { size, .. } => {
                Ray::from_orthographic(normalized_screen_position, aspect, size, camera)
            }
        }
    }

    /// Create a new [`Ray`] from a perspective projection
    fn from_perspective(
        screen_position: Vector2f,
        aspect: f32,
        fov_y: f32,
        camera: &Camera,
    ) -> Ray {
        let half_fov = (fov_y * 0.5).tan();

        let x = (2.0 * screen_position.x - 1.0) * half_fov * aspect;
        let y = (1.0 - 2.0 * screen_position.y) * half_fov;

        Ray::new(
            camera.position,
            camera.rotation.forward() + camera.rotation.right() * x + camera.rotation.up() * y,
        )
    }

    /// Create a new [`Ray`] from an orthographic projection
    fn from_orthographic(
        screen_position: Vector2f,
        aspect: f32,
        size: f32,
        camera: &Camera,
    ) -> Ray {
        let x = (2.0 * screen_position.x - 1.0) * size;
        let y = (1.0 - 2.0 * screen_position.y) * size / aspect;

        Ray::new(
            camera.position + x * camera.rotation.right() + y * camera.rotation.up(),
            camera.rotation.forward(),
        )
    }
}
