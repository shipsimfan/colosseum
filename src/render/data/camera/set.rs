use crate::render::data::CameraRenderData;
use alexandria::math::{Matrix4x4f, Vector3f};

impl CameraRenderData {
    /// Set the camera data
    pub fn set(&mut self, view_projection: Matrix4x4f, position: Vector3f) {
        self.view_projection = view_projection;
        self.position = position;
    }
}
