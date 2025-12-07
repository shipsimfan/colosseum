use crate::{Result, Transforms, graphics::MeshRenderer};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl MeshRenderer {
    /// Draw this mesh using the active settings
    pub(in crate::graphics) fn draw(
        &mut self,
        transforms: &mut Transforms,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        // Skip rendering if theres nothing to render
        if self.num_instances() == 0 {
            return Ok(());
        }

        // Update transforms if needed
        for (i, (instance, epoch)) in self.instances.iter_mut().enumerate() {
            let transform = &mut transforms[*instance];
            let new_epoch = transform.update();
            if new_epoch != *epoch {
                self.instance_buffer[i] = transform.matrix();
                *epoch = new_epoch;
            }
        }

        // Bind the buffers
        self.mesh.bind(device, device_context)?;
        self.instance_buffer.bind(device_context)?;

        // Draw
        device_context.draw_indexed_instanced(
            self.mesh.indices().len() as _,
            self.num_instances() as _,
            0,
            0,
            0,
        );
        Ok(())
    }
}
