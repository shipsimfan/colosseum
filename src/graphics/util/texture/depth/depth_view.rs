use crate::graphics::util::DepthTexture;
use win32::d3d11::ID3D11DepthStencilView;

impl DepthTexture {
    /// Get a view of the underlying texture for rendering depth information into
    pub fn depth_view(&mut self) -> *mut ID3D11DepthStencilView {
        self.view.as_mut()
    }
}
