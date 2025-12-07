use crate::{Result, graphics::MeshRenderer};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl MeshRenderer {
    /// Draw this mesh using the active settings
    pub(in crate::graphics) fn draw(
        &mut self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        // Skip rendering if theres nothing to render
        if self.active_instances == 0 {
            return Ok(());
        }

        // Bind the buffers
        self.mesh.bind(device, device_context)?;
        self.instance_buffer.bind(device_context)?;

        // Draw
        device_context.draw_indexed_instanced(
            self.mesh.indices().len() as _,
            self.active_instances as _,
            0,
            0,
            0,
        );
        Ok(())
    }
}
