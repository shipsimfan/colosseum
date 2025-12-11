use crate::{
    Result, Transform,
    graphics::{
        Camera, CameraPostProcessing, CameraProjection, managed_objects::camera::CameraCbContent,
        util::ConstantBuffer,
    },
    math::{Matrix4x4f, Vector2u},
};
use win32::d3d11::{D3D11_VIEWPORT, ID3D11Device};

impl Camera {
    /// Create a new [`Camera`]
    pub(in crate::graphics::managed_objects::camera) fn new(
        projection: CameraProjection,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Create math elements
        let mut transform = Transform::new();
        let transform_epoch = transform.update_camera();
        let projection_matrix = Matrix4x4f::IDENTITY;

        // Create constant buffer
        let buffer_content = CameraCbContent::new(Matrix4x4f::IDENTITY);
        let buffer = ConstantBuffer::new(buffer_content, ConstantBuffer::CAMERA_SLOT, device)?;

        // Create relative viewport
        let relative_viewport = D3D11_VIEWPORT {
            top_left_x: 0.0,
            top_left_y: 0.0,
            width: 1.0,
            height: 1.0,
            ..Default::default()
        };

        // Create post processing
        let post_processing =
            CameraPostProcessing::new(Vector2u::new(1, 1), &relative_viewport, 1.0, None, device)?;

        Ok(Camera {
            active: true,
            transform,
            transform_epoch,
            projection,
            projection_dirty: true,
            projection_matrix,
            buffer,
            relative_viewport,
            viewport_dirty: true,
            post_processing,
        })
    }
}
