use crate::{
    Error, Result,
    graphics::{MaterialInner, Shader},
    math::{Color3f, Vector4f},
};
use std::rc::Rc;
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_CPU_ACCESS_FLAG, D3D11_SUBRESOURCE_DATA,
        D3D11_USAGE, ID3D11Device,
    },
    try_hresult,
};

impl MaterialInner {
    /// Create a new [`MaterialInner`]
    pub(in crate::graphics) fn new(
        shader: Rc<Shader>,
        color: Color3f,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Create constant buffer
        let color4 = Vector4f::new(color.r, color.b, color.g, 1.0);
        let buffer_desc = D3D11_BUFFER_DESC {
            byte_width: std::mem::size_of::<Vector4f>() as _,
            usage: D3D11_USAGE::Dynamic,
            bind_flags: D3D11_BIND_FLAG::ConstantBuffer as _,
            cpu_access_flags: D3D11_CPU_ACCESS_FLAG::Write as _,
            misc_flags: 0,
            structure_byte_stride: 0,
        };
        let initial_data = D3D11_SUBRESOURCE_DATA {
            sys_mem: &color4 as *const _ as _,
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        };

        let buffer = ComPtr::new_in(|buffer| {
            try_hresult!(device.create_buffer(&buffer_desc, &initial_data, buffer))
        })
        .map_err(|error| Error::new_inner("unable to create material constant buffer", error))?;

        Ok(MaterialInner {
            shader,
            color,
            dirty: false,
            buffer,
        })
    }
}
