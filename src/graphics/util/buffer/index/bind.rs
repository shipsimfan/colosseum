use crate::graphics::util::IndexBuffer;
use win32::{
    d3d11::{ID3D11Buffer, ID3D11DeviceContext},
    dxgi::DXGI_FORMAT,
};

impl IndexBuffer {
    /// Bind this index buffer to be the active one on the `device_context`
    pub fn bind(&self, device_context: &mut ID3D11DeviceContext) {
        device_context.ia_set_index_buffer(
            self.buffer.as_ref() as *const ID3D11Buffer as _,
            DXGI_FORMAT::R32UInt,
            0,
        );
    }
}
