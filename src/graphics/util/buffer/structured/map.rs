use crate::{Error, Result, graphics::util::StructuredBuffer};
use std::num::NonZeroUsize;
use win32::{
    d3d11::{D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, ID3D11Device, ID3D11DeviceContext},
    try_hresult,
};

impl<T: Sized + Copy> StructuredBuffer<T> {
    /// Set the contents of this buffer to `iter`
    pub fn map<I: Iterator<Item = T>>(
        &mut self,
        count: usize,
        mut iter: I,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        if count > self.buffer_capacity {
            // Resize the capacity
            while count > self.buffer_capacity {
                self.buffer_capacity *= 2;
            }

            // Recreate the buffer and view
            *self = StructuredBuffer::new(
                NonZeroUsize::new(self.buffer_capacity).unwrap(),
                self.slot,
                device,
            )?;
        }

        // Remap the elements
        let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
        try_hresult!(device_context.map(
            self.buffer.as_mut(),
            0,
            D3D11_MAP::WriteDiscard,
            0,
            &mut mapped_resource,
        ))
        .map_err(|error| Error::new_inner("unable to map instance buffer", error))?;

        let dest = unsafe { std::slice::from_raw_parts_mut(mapped_resource.data as *mut T, count) };
        for i in 0..count {
            dest[i] = iter.next().unwrap();
        }

        device_context.unmap(self.buffer.as_mut(), 0);
        Ok(())
    }
}
