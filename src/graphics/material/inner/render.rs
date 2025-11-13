use crate::{Error, Result, graphics::MaterialInner, math::Vector4f};
use win32::{
    d3d11::{D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, ID3D11DeviceContext},
    try_hresult,
};

impl MaterialInner {
    /// Binds the material properties for rendering and draws the registered meshes
    pub fn render(&mut self, device_context: &mut ID3D11DeviceContext) -> Result<()> {
        let mut bound = false;
        for mesh_renderer in &self.mesh_renderers {
            let mut renderer = mesh_renderer.borrow_mut();
            if !renderer.is_active() {
                continue;
            }

            if !bound {
                // Update constant buffer if needed
                if self.dirty {
                    let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
                    try_hresult!(device_context.map(
                        self.buffer.as_mut(),
                        0,
                        D3D11_MAP::WriteDiscard,
                        0,
                        &mut mapped_resource,
                    ))
                    .map_err(|error| {
                        Error::new_inner("unable to map material constant buffer", error)
                    })?;

                    let color4 = Vector4f::new(self.color.r, self.color.b, self.color.g, 1.0);
                    *unsafe { &mut *(mapped_resource.data as *mut _) } = color4;

                    device_context.unmap(self.buffer.as_mut(), 0);

                    self.dirty = false;
                }

                // Set the material property buffer
                let buffer = self.buffer.as_mut() as *mut _;
                device_context.vs_set_constant_buffers(1, 1, &buffer);

                bound = true;
            }

            renderer.draw(device_context);
        }
        Ok(())
    }
}
