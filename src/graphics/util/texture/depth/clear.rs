use crate::graphics::util::DepthTexture;
use win32::d3d11::{D3D11_CLEAR_FLAG, ID3D11DeviceContext};

impl DepthTexture {
    /// Clear the texture to a single depth
    pub fn clear(&mut self, depth: f32, device_context: &mut ID3D11DeviceContext) {
        device_context.clear_depth_stencil_view(
            self.view.as_mut(),
            D3D11_CLEAR_FLAG::Depth as _,
            depth,
            0,
        );
    }
}
