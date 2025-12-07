use crate::{Error, Result, graphics::util::InstanceBuffer};
use win32::{
    d3d11::{D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, ID3D11DeviceContext},
    try_hresult,
};

impl<T: Sized + Copy> InstanceBuffer<T> {
    /// Bind this instance buffer to be an active one on the `device_context`
    pub fn bind(&mut self, device_context: &mut ID3D11DeviceContext) -> Result<()> {
        // Update the instance buffer if needed
        if self.dirty {
            let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
            try_hresult!(device_context.map(
                self.buffer.as_mut(),
                0,
                D3D11_MAP::WriteDiscard,
                0,
                &mut mapped_resource,
            ))
            .map_err(|error| Error::new_inner("unable to map instance buffer", error))?;

            let dest = unsafe {
                std::slice::from_raw_parts_mut(mapped_resource.data as *mut T, self.content.len())
            };
            dest.copy_from_slice(&self.content);
            device_context.unmap(self.buffer.as_mut(), 0);
            self.dirty = false;
        }

        // Bind the instance buffer
        let buffer = self.buffer.as_mut() as *mut _;
        let stride = std::mem::size_of::<T>() as _;
        let offset = 0;
        device_context.ia_set_vertex_buffers(self.slot, 1, &buffer, &stride, &offset);

        Ok(())
    }
}
