use crate::render::Camera;
use alexandria::math::Matrix4x4f;

impl Camera {
    /// Get the inverse view matrix of the camera
    pub fn inverse_view_matrix(&self) -> Matrix4x4f {
        Matrix4x4f::from_translation(self.position) * Matrix4x4f::from_rotation(self.rotation)
    }
}
