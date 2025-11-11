use crate::{
    Error, Result,
    graphics::{CameraInner, CameraProjection, Transform},
    math::{Matrix4x4f, Vector2u},
};
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_CPU_ACCESS_FLAG, D3D11_SUBRESOURCE_DATA,
        D3D11_USAGE, D3D11_VIEWPORT, ID3D11Device,
    },
    try_hresult,
};

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
        let buffer_desc = D3D11_BUFFER_DESC {
            byte_width: std::mem::size_of::<Matrix4x4f>() as _,
            usage: D3D11_USAGE::Dynamic,
            bind_flags: D3D11_BIND_FLAG::ConstantBuffer as _,
            cpu_access_flags: D3D11_CPU_ACCESS_FLAG::Write as _,
            misc_flags: 0,
            structure_byte_stride: 0,
        };
        let initial_data = D3D11_SUBRESOURCE_DATA {
            sys_mem: &combined_matrix as *const _ as _,
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        };

        let buffer = ComPtr::new_in(|buffer| {
            try_hresult!(device.create_buffer(&buffer_desc, &initial_data, buffer))
        })
        .map_err(|error| Error::new_inner("unable to create camera constant buffer", error))?;

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
