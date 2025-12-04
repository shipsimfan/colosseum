use crate::{
    Result,
    graphics::{
        MeshInner,
        util::{IndexBuffer, VertexBuffer},
    },
};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl MeshInner {
    /// Set this mesh as the active mesh for rendering
    pub(in crate::graphics) fn bind(
        &self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        let (vertex_buffer, index_buffer) = self.buffers.get_or_try_init(|| {
            Ok((
                VertexBuffer::new(&self.vertices, 0, device)?,
                IndexBuffer::new(&self.indices, device)?,
            ))
        })?;

        vertex_buffer.bind(device_context);
        index_buffer.bind(device_context);
        Ok(())
    }
}
