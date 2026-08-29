use crate::render::CameraRenderData;
use alexandria::math::{Matrix4x4f, Vector3f};

impl CameraRenderData {
    /// Set the camera's data
    pub fn set(&mut self, view_projection: Matrix4x4f, position: Vector3f) {
        self.shader_data[0].set(view_projection, position);
    }
}
