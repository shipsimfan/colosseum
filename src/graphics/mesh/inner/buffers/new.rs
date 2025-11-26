use crate::{
    Error, Result,
    graphics::{Vertex, mesh::inner::MeshBuffers},
};
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_SUBRESOURCE_DATA, D3D11_USAGE, ID3D11Device,
    },
    try_hresult,
};

impl MeshBuffers {
    /// Create a new set of [`MeshBuffers`]
    pub fn new(vertices: &[Vertex], indices: &[u32], device: &ID3D11Device) -> Result<Self> {
        // Create the vertex buffer
        let vertex_buffer_desc = D3D11_BUFFER_DESC {
            byte_width: (vertices.len() * std::mem::size_of::<Vertex>()) as _,
            usage: D3D11_USAGE::Immutable,
            bind_flags: D3D11_BIND_FLAG::VertexBuffer as _,
            cpu_access_flags: 0,
            misc_flags: 0,
            structure_byte_stride: std::mem::size_of::<Vertex>() as _,
        };
        let vertex_data = D3D11_SUBRESOURCE_DATA {
            sys_mem: vertices.as_ptr().cast(),
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        };
        let vertex_buffer = ComPtr::new_in(|vertex_buffer| {
            try_hresult!(device.create_buffer(&vertex_buffer_desc, &vertex_data, vertex_buffer))
        })
        .map_err(|error| Error::new_inner("unable to create vertex buffer", error))?;

        // Create the index buffer
        let index_buffer_desc = D3D11_BUFFER_DESC {
            byte_width: (indices.len() * std::mem::size_of::<u32>()) as _,
            usage: D3D11_USAGE::Immutable,
            bind_flags: D3D11_BIND_FLAG::IndexBuffer as _,
            cpu_access_flags: 0,
            misc_flags: 0,
            structure_byte_stride: std::mem::size_of::<u32>() as _,
        };
        let index_data = D3D11_SUBRESOURCE_DATA {
            sys_mem: indices.as_ptr().cast(),
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        };
        let index_buffer = ComPtr::new_in(|index_buffer| {
            try_hresult!(device.create_buffer(&index_buffer_desc, &index_data, index_buffer))
        })
        .map_err(|error| Error::new_inner("unable to create index buffer", error))?;

        Ok(MeshBuffers {
            vertex_buffer,
            index_buffer,
        })
    }
}
