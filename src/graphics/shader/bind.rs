use crate::graphics::Shader;
use std::ptr::null;
use win32::d3d11::{ID3D11DeviceContext, ID3D11InputLayout, ID3D11PixelShader, ID3D11VertexShader};

impl Shader {
    /// Set this shader as the active shader for rendering
    pub(in crate::graphics) fn bind(&self, device_context: &mut ID3D11DeviceContext) {
        device_context.vs_set_shader(
            self.vertex_shader.as_ref() as *const ID3D11VertexShader as _,
            null(),
            0,
        );
        device_context.ps_set_shader(
            self.pixel_shader.as_ref() as *const ID3D11PixelShader as _,
            null(),
            0,
        );
        device_context
            .ia_set_input_layout(self.input_layout.as_ref() as *const ID3D11InputLayout as _);
    }
}
