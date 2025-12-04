use crate::{Error, Result, graphics::util::VertexBuffer};
use std::marker::PhantomData;
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_SUBRESOURCE_DATA, D3D11_USAGE, ID3D11Device,
    },
    try_hresult,
};

impl<Vertex> VertexBuffer<Vertex> {
    /// Create a new [`VertexBuffer`]
    pub fn new(vertices: &[Vertex], slot: u32, device: &ID3D11Device) -> Result<Self> {
        let buffer_desc = D3D11_BUFFER_DESC {
            byte_width: (vertices.len() * std::mem::size_of::<Vertex>()) as _,
            usage: D3D11_USAGE::Immutable,
            bind_flags: D3D11_BIND_FLAG::VertexBuffer as _,
            cpu_access_flags: 0,
            misc_flags: 0,
            structure_byte_stride: std::mem::size_of::<Vertex>() as _,
        };
        let data = D3D11_SUBRESOURCE_DATA {
            sys_mem: vertices.as_ptr().cast(),
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        };
        let buffer = ComPtr::new_in(|buffer| {
            try_hresult!(device.create_buffer(&buffer_desc, &data, buffer))
        })
        .map_err(|error| Error::new_inner("unable to create vertex buffer", error))?;

        Ok(VertexBuffer {
            buffer,
            slot,
            _vertex: PhantomData,
        })
    }
}
