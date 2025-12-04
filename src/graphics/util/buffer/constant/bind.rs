use crate::{Error, Result, graphics::util::ConstantBuffer};
use win32::{
    d3d11::{D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, ID3D11DeviceContext},
    try_hresult,
};

impl<T: Sized + Copy> ConstantBuffer<T> {
    /// Bind this constant buffer to be an active one on the `device_context`
    pub fn bind(&mut self, device_context: &mut ID3D11DeviceContext) -> Result<()> {
        // Update the buffer if needed
        if self.dirty {
            let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
            try_hresult!(device_context.map(
                self.buffer.as_mut(),
                0,
                D3D11_MAP::WriteDiscard,
                0,
                &mut mapped_resource,
            ))
            .map_err(|error| Error::new_inner("unable to map constant buffer", error))?;

            *unsafe { &mut *(mapped_resource.data as *mut _) } = self.content;

            device_context.unmap(self.buffer.as_mut(), 0);

            self.dirty = false;
        }

        // Actually bind the buffer
        let buffer = self.buffer.as_mut() as *mut _;
        device_context.vs_set_constant_buffers(self.slot, 1, &buffer);
        device_context.ps_set_constant_buffers(self.slot, 1, &buffer);

        Ok(())
    }
}
