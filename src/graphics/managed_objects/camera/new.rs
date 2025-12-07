use crate::{
    Result,
    graphics::{
        Camera, CameraProjection, managed_objects::camera::CameraCbContent, util::ConstantBuffer,
    },
    math::{Matrix4x4f, Transform},
};
use win32::d3d11::{D3D11_VIEWPORT, ID3D11Device};

impl Camera {
    /// Create a new [`Camera`]
    pub(in crate::graphics::managed_objects::camera) fn new(
        projection: CameraProjection,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Create combined matrix
        let transform = Transform::new();
        let projection_matrix = Matrix4x4f::IDENTITY;

        // Create constant buffer
        let buffer_content = CameraCbContent::new(Matrix4x4f::IDENTITY);
        let buffer = ConstantBuffer::new(buffer_content, 0, device)?;

        Ok(Camera {
            active: true,
            transform,
            projection,
            projection_dirty: true,
            projection_matrix,
            buffer,
            relative_viewport: D3D11_VIEWPORT {
                top_left_x: 0.0,
                top_left_y: 0.0,
                width: 1.0,
                height: 1.0,
                ..Default::default()
            },
            screen_viewport: D3D11_VIEWPORT {
                top_left_x: 0.0,
                top_left_y: 0.0,
                width: 0.0,
                height: 0.0,
                ..Default::default()
            },
            viewport_dirty: true,
        })
    }
}
