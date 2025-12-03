use crate::graphics::util::VertexShader;
use std::ptr::null;
use win32::d3d11::{ID3D11DeviceContext, ID3D11InputLayout, ID3D11VertexShader};

impl VertexShader {
    /// Bind this to be the active vertex shader
    pub fn bind(&self, device_context: &mut ID3D11DeviceContext) {
        device_context.vs_set_shader(
            self.shader.as_ref() as *const ID3D11VertexShader as _,
            null(),
            0,
        );
        device_context
            .ia_set_input_layout(self.input_layout.as_ref() as *const ID3D11InputLayout as _);
    }
}
