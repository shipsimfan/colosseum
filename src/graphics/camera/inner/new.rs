use crate::{
    Result,
    graphics::{
        CameraInner, CameraProjection, camera::inner::CameraCbContent, util::ConstantBuffer,
    },
    math::{Transform, Vector2u},
};
use win32::d3d11::{D3D11_VIEWPORT, ID3D11Device};

impl CameraInner {
    /// Create a new [`CameraInner`]
    pub(in crate::graphics::camera) fn new(
        projection: CameraProjection,
        screen_size: Vector2u,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Create combined matrix
        let transform = Transform::new();
        let projection_matrix = projection.matrix(screen_size);
        let combined_matrix = transform.matrix() * projection_matrix;

        // Create constant buffer
        let buffer_content = CameraCbContent::new(combined_matrix);
        let buffer = ConstantBuffer::new(buffer_content, 0, device)?;

        Ok(CameraInner {
            active: true,
            transform,
            projection,
            projection_dirty: false,
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
                width: screen_size.x as _,
                height: screen_size.y as _,
                ..Default::default()
            },
            viewport_dirty: false,
        })
    }
}
