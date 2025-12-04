use crate::{Error, Result, graphics::util::ConstantBuffer};
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_CPU_ACCESS_FLAG, D3D11_SUBRESOURCE_DATA,
        D3D11_USAGE, ID3D11Device,
    },
    try_hresult,
};

impl<T: Sized + Copy> ConstantBuffer<T> {
    /// Create a new [`ConstantBuffer`]
    pub fn new(content: T, slot: u32, device: &ID3D11Device) -> Result<Self> {
        let desc = D3D11_BUFFER_DESC {
            byte_width: std::mem::size_of::<T>() as _,
            usage: D3D11_USAGE::Dynamic,
            bind_flags: D3D11_BIND_FLAG::ConstantBuffer as _,
            cpu_access_flags: D3D11_CPU_ACCESS_FLAG::Write as _,
            misc_flags: 0,
            structure_byte_stride: 0,
        };
        let data = D3D11_SUBRESOURCE_DATA {
            sys_mem: &content as *const _ as _,
            sys_mem_pitch: 0,
            sys_mem_slice_pitch: 0,
        };

        let buffer =
            ComPtr::new_in(|buffer| try_hresult!(device.create_buffer(&desc, &data, buffer)))
                .map_err(|error| {
                    Error::new_inner("unable to create material constant buffer", error)
                })?;

        Ok(ConstantBuffer {
            buffer,
            content,
            dirty: false,
            slot,
        })
    }
}
