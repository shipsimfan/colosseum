use crate::graphics::util::BackBufferTexture;
use win32::d3d11::ID3D11RenderTargetView;

impl BackBufferTexture {
    /// Get a view of the underlying texture for rendering into
    pub fn render_view(&mut self) -> *mut ID3D11RenderTargetView {
        self.view.as_mut()
    }
}
