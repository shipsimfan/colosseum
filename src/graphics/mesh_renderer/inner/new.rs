use crate::{
    Error, Result,
    graphics::{Material, Mesh, MeshRendererInner},
    math::Matrix4x4f,
};
use std::ptr::null;
use win32::{
    ComPtr,
    d3d11::{D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_CPU_ACCESS_FLAG, D3D11_USAGE, ID3D11Device},
    try_hresult,
};

impl MeshRendererInner {
    /// Create a new [`MeshRendererInner`]
    pub(in crate::graphics::mesh_renderer) fn new(
        material: Material,
        mesh: Mesh,
        max_instances: usize,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let instance_buffer_desc = D3D11_BUFFER_DESC {
            byte_width: (max_instances * std::mem::size_of::<Matrix4x4f>()) as _,
            usage: D3D11_USAGE::Dynamic,
            bind_flags: D3D11_BIND_FLAG::VertexBuffer as _,
            cpu_access_flags: D3D11_CPU_ACCESS_FLAG::Write as _,
            misc_flags: 0,
            structure_byte_stride: std::mem::size_of::<Matrix4x4f>() as _,
        };
        let instance_buffer = ComPtr::new_in(|instance_buffer| {
            try_hresult!(device.create_buffer(&instance_buffer_desc, null(), instance_buffer))
        })
        .map_err(|error| Error::new_inner("unable to create instance buffer", error))?;

        Ok(MeshRendererInner {
            active: true,
            material,
            mesh,
            instances: Vec::with_capacity(max_instances),
            dirty: false,
            max_instances,
            instance_buffer,
        })
    }
}
