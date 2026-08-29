use crate::render::data::camera::CameraShaderData;
use alexandria::math::{Matrix4x4f, Vector3f};

impl CameraShaderData {
    /// Set the camera data
    pub fn set(&mut self, view_projection: Matrix4x4f, position: Vector3f) {
        self.view_projection = view_projection;
        self.position = position;
    }
}
