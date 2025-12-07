use crate::{Error, Result, graphics::util::InstanceBuffer};
use std::ptr::null;
use win32::{
    ComPtr,
    d3d11::{D3D11_BIND_FLAG, D3D11_BUFFER_DESC, D3D11_CPU_ACCESS_FLAG, D3D11_USAGE, ID3D11Device},
    try_hresult,
};

impl<T: Sized + Copy> InstanceBuffer<T> {
    /// Create a new [`InstanceBuffer`]
    pub fn new(default: T, count: usize, slot: u32, device: &ID3D11Device) -> Result<Self> {
        let content = vec![default; count].into_boxed_slice();

        let desc = D3D11_BUFFER_DESC {
            byte_width: (count * std::mem::size_of::<T>()) as _,
            usage: D3D11_USAGE::Dynamic,
            bind_flags: D3D11_BIND_FLAG::VertexBuffer as _,
            cpu_access_flags: D3D11_CPU_ACCESS_FLAG::Write as _,
            misc_flags: 0,
            structure_byte_stride: std::mem::size_of::<T>() as _,
        };
        let buffer =
            ComPtr::new_in(|buffer| try_hresult!(device.create_buffer(&desc, null(), buffer)))
                .map_err(|error| Error::new_inner("unable to create instance buffer", error))?;

        Ok(InstanceBuffer {
            buffer,
            content,
            dirty: false,
            slot,
        })
    }
}
