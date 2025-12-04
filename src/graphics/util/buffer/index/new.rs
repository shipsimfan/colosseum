use crate::{Error, Result, graphics::util::IndexBuffer};
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_SUBRESOURCE_DATA, D3D11_USAGE, ID3D11Device,
    },
    try_hresult,
};

impl IndexBuffer {
    /// Create a new [`IndexBuffer`]
    pub fn new(indices: &[u32], device: &ID3D11Device) -> Result<Self> {
        let buffer_desc = D3D11_BUFFER_DESC {
            byte_width: (indices.len() * std::mem::size_of::<u32>()) as _,
            usage: D3D11_USAGE::Immutable,
            bind_flags: D3D11_BIND_FLAG::IndexBuffer as _,
            cpu_access_flags: 0,
            misc_flags: 0,
            structure_byte_stride: std::mem::size_of::<u32>() as _,
        };
        let data = D3D11_SUBRESOURCE_DATA {
            sys_mem: indices.as_ptr().cast(),
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        };
        let buffer = ComPtr::new_in(|buffer| {
            try_hresult!(device.create_buffer(&buffer_desc, &data, buffer))
        })
        .map_err(|error| Error::new_inner("unable to create index buffer", error))?;

        Ok(IndexBuffer { buffer })
    }
}
