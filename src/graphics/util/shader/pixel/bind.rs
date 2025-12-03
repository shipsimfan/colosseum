use crate::graphics::util::PixelShader;
use std::ptr::null;
use win32::d3d11::{ID3D11DeviceContext, ID3D11PixelShader};

impl PixelShader {
    /// Bind this to be the active pixel shader
    pub fn bind(&self, device_context: &mut ID3D11DeviceContext) {
        device_context.ps_set_shader(
            self.shader.as_ref() as *const ID3D11PixelShader as _,
            null(),
            0,
        );
    }
}
