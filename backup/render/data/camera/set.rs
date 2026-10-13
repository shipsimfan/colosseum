use crate::render::RenderCamera;
use alexandria::math::{Matrix4x4f, Vector3f};

impl RenderCamera {
    /// Set the camera data
    pub fn set(&mut self, view_projection: Matrix4x4f, position: Vector3f) {
        self.view_projection = view_projection;
        self.position = position;
    }
}
