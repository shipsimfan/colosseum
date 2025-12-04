use crate::{Result, graphics::MaterialInner};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl MaterialInner {
    /// Binds the material properties for rendering and draws the registered meshes
    pub fn render(
        &mut self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        let mut buffer_bound = false;
        for mesh_renderer in &self.mesh_renderers {
            let mut renderer = mesh_renderer.borrow_mut();
            if !renderer.is_active() {
                continue;
            }

            if !buffer_bound {
                self.buffer.bind(device_context)?;
                buffer_bound = true;
            }

            renderer.draw(device, device_context)?;
        }
        Ok(())
    }
}
