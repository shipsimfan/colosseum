use crate::graphics::util::BackBufferTexture;
use win32::d3d11::ID3D11DeviceContext;

impl BackBufferTexture {
    /// Clear the texture to a single color
    pub fn clear(&mut self, color: [f32; 4], device_context: &mut ID3D11DeviceContext) {
        device_context.clear_render_target_view(self.view.as_mut(), color);
    }
}
