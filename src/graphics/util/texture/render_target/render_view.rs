use crate::graphics::util::RenderTargetTexture;
use win32::d3d11::ID3D11RenderTargetView;

impl RenderTargetTexture {
    /// Get a view of the underlying texture for rendering into
    pub fn render_view(&mut self) -> *mut ID3D11RenderTargetView {
        self.render_view.as_mut()
    }
}
