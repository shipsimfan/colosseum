use crate::render::CameraRenderData;
use alexandria::math::Matrix4x4f;

impl CameraRenderData {
    /// Set the camera's view-projection matrix
    pub fn set_view_projection(&mut self, view_projection: Matrix4x4f) {
        self.shader_data[0].set_view_projection(view_projection);
    }
}
