use crate::{
    Result,
    graphics::{MeshInner, mesh::inner::MeshBuffers},
};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl MeshInner {
    /// Set this mesh as the active mesh for rendering
    pub(in crate::graphics) fn bind(
        &self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        let buffers = self
            .buffers
            .get_or_try_init(|| MeshBuffers::new(&self.vertices, &self.indices, device))?;

        buffers.bind(device_context);
        Ok(())
    }
}
