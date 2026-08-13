use crate::update::components::{Camera, CameraProjection};

impl Camera {
    /// Set the projection for the camera
    pub fn set_projection(&mut self, projection: CameraProjection) {
        self.projection = projection;
        self.projection_dirty = true;
    }

    /// Set the projection for the camera to a perspective projection
    pub fn set_perspective(&mut self, fov_y: f32, near: f32, far: f32) {
        self.set_projection(CameraProjection::Perspective { fov_y, near, far });
    }

    /// Set the projection for the camera to an infinite perspective projection
    pub fn set_infinite_perspective(&mut self, fov_y: f32, near: f32) {
        self.set_projection(CameraProjection::InfinitePerspective { fov_y, near });
    }

    /// Set the projection for the camera to an orthographic projection
    pub fn set_orthographic(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) {
        self.set_projection(CameraProjection::Orthographic {
            left,
            right,
            bottom,
            top,
            near,
            far,
        });
    }
}
