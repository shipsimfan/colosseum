use crate::graphics::util::VertexBuffer;
use win32::d3d11::{ID3D11Buffer, ID3D11DeviceContext};

impl<Vertex> VertexBuffer<Vertex> {
    /// Bind this vertex buffer to be an active one on the `device_context`
    pub fn bind(&self, device_context: &mut ID3D11DeviceContext) {
        let vertex_buffer = self.buffer.as_ref() as *const ID3D11Buffer as _;
        let stride = std::mem::size_of::<Vertex>() as _;
        let offset = 0;
        device_context.ia_set_vertex_buffers(self.slot, 1, &vertex_buffer, &stride, &offset);
    }
}
