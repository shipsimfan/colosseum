use crate::graphics::{Vertex, mesh::inner::MeshBuffers};
use win32::{
    d3d11::{ID3D11Buffer, ID3D11DeviceContext},
    dxgi::DXGI_FORMAT,
};

impl MeshBuffers {
    /// Set this mesh as the active mesh for rendering
    pub(in crate::graphics::mesh::inner) fn bind(&self, device_context: &mut ID3D11DeviceContext) {
        let vertex_buffer = self.vertex_buffer.as_ref() as *const ID3D11Buffer as _;
        let stride = std::mem::size_of::<Vertex>() as _;
        let offset = 0;
        device_context.ia_set_vertex_buffers(0, 1, &vertex_buffer, &stride, &offset);
        device_context.ia_set_index_buffer(
            self.index_buffer.as_ref() as *const ID3D11Buffer as _,
            DXGI_FORMAT::R32UInt,
            0,
        );
    }
}
