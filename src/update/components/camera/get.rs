use crate::update::components::{Camera, CameraProjection};
use alexandria::math::{Matrix4x4f, Vector2u, Vector3f};

impl Camera {
    /// Get the current projection type for this camera
    pub fn projection(&self) -> &CameraProjection {
        &self.projection
    }

    /// Get the projection matrix, shadow corners, and depth for this camera
    pub(in crate::update) fn projection_matrix(
        &mut self,
        viewport_size: Vector2u,
    ) -> (Matrix4x4f, [([Vector3f; 4], f32); 5]) {
        if self.projection_dirty || self.last_viewport_size != viewport_size {
            let aspect = viewport_size.x as f32 / viewport_size.y as f32;

            self.projection_matrix = self.projection.matrix(aspect);
            self.corners = self.projection.shadow_corners(aspect);

            self.last_viewport_size = viewport_size;
            self.projection_dirty = false;
        }

        (self.projection_matrix, self.corners)
    }
}
