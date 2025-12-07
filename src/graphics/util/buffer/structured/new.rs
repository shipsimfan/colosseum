use crate::{Error, Result, graphics::util::StructuredBuffer};
use std::{marker::PhantomData, num::NonZeroUsize, ptr::null};
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_BUFFEREX_SRV, D3D11_CPU_ACCESS_FLAG,
        D3D11_RESOURCE_MISC_FLAG, D3D11_SHADER_RESOURCE_VIEW_DESC,
        D3D11_SHADER_RESOURCE_VIEW_DESC_UNION, D3D11_SRV_DIMENSION, D3D11_USAGE, ID3D11Device,
    },
    dxgi::DXGI_FORMAT,
    try_hresult,
};

impl<T: Sized + Copy> StructuredBuffer<T> {
    /// Create a new [`StructuredBuffer`] with initial `capacity`
    pub fn new(capacity: NonZeroUsize, slot: u32, device: &ID3D11Device) -> Result<Self> {
        let capacity = capacity.get();

        // Create GPU buffer
        let desc = D3D11_BUFFER_DESC {
            byte_width: (std::mem::size_of::<T>() * capacity) as _,
            usage: D3D11_USAGE::Dynamic,
            bind_flags: D3D11_BIND_FLAG::ShaderResource as _,
            cpu_access_flags: D3D11_CPU_ACCESS_FLAG::Write as _,
            misc_flags: D3D11_RESOURCE_MISC_FLAG::BufferStructured as _,
            structure_byte_stride: std::mem::size_of::<T>() as _,
        };

        let mut buffer =
            ComPtr::new_in(|buffer| try_hresult!(device.create_buffer(&desc, null(), buffer)))
                .map_err(|error| Error::new_inner("unable to create structured buffer", error))?;

        // Create GPU buffer view
        let shader_resource_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
            format: DXGI_FORMAT::Unknown,
            view_dimension: D3D11_SRV_DIMENSION::BufferEx,
            u: D3D11_SHADER_RESOURCE_VIEW_DESC_UNION {
                buffer_ex: D3D11_BUFFEREX_SRV {
                    first_element: 0,
                    num_elements: capacity as _,
                    flags: 0,
                },
            },
        };

        let view = ComPtr::new_in(|view| {
            try_hresult!(device.create_shader_resource_view(
                buffer.as_mut(),
                &shader_resource_desc,
                view
            ))
        })
        .map_err(|error| Error::new_inner("unable to create structured buffer view", error))?;

        Ok(StructuredBuffer {
            buffer,
            buffer_capacity: capacity,
            view,
            slot,
            _type: PhantomData,
        })
    }
}
