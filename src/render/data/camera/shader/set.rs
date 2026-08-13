use crate::render::data::camera::CameraShaderData;
use alexandria::math::Matrix4x4f;

impl CameraShaderData {
    /// Set the camera's view-projection matrix
    pub fn set_view_projection(&mut self, view_projection: Matrix4x4f) {
        self.view_projection = view_projection;
    }
}
