use crate::{
    Error, Result,
    graphics::context::{LightType, managed_objects::lights::LightList},
};
use std::{cell::RefCell, ptr::null, rc::Rc};
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

impl<T: LightType> LightList<T> {
    /// Create a new [`LightList`]
    pub fn new(buffer_slot: u32, initial_capacity: usize, device: &ID3D11Device) -> Result<Self> {
        // Create shared list
        let shared_list = Rc::new(RefCell::new((Vec::with_capacity(initial_capacity), false)));

        // Create buffer cache
        let buffer_cache = Vec::with_capacity(initial_capacity);

        // Create GPU buffer
        let buffer_desc = D3D11_BUFFER_DESC {
            byte_width: (std::mem::size_of::<T::GPU>() * initial_capacity) as _,
            usage: D3D11_USAGE::Dynamic,
            bind_flags: D3D11_BIND_FLAG::ShaderResource as _,
            cpu_access_flags: D3D11_CPU_ACCESS_FLAG::Write as _,
            misc_flags: D3D11_RESOURCE_MISC_FLAG::BufferStructured as _,
            structure_byte_stride: std::mem::size_of::<T::GPU>() as _,
        };

        let mut buffer = ComPtr::new_in(|buffer| {
            try_hresult!(device.create_buffer(&buffer_desc, null(), buffer))
        })
        .map_err(|error| Error::new_inner("unable to create light list buffer", error))?;

        // Create GPU buffer view
        let shader_resource_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
            format: DXGI_FORMAT::Unknown,
            view_dimension: D3D11_SRV_DIMENSION::BufferEx,
            u: D3D11_SHADER_RESOURCE_VIEW_DESC_UNION {
                buffer_ex: D3D11_BUFFEREX_SRV {
                    first_element: 0,
                    num_elements: initial_capacity as _,
                    flags: 0,
                },
            },
        };

        let buffer_view = ComPtr::new_in(|shader_resource_view| {
            try_hresult!(device.create_shader_resource_view(
                buffer.as_mut(),
                &shader_resource_desc,
                shader_resource_view
            ))
        })
        .map_err(|error| Error::new_inner("unable to create light list buffer view", error))?;

        // Create light list
        Ok(LightList {
            shared_list,
            buffer_slot,
            buffer_cache,
            buffer_capacity: initial_capacity,
            buffer,
            buffer_view,
        })
    }
}
