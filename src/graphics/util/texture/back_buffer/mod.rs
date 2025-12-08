use win32::{ComPtr, d3d11::ID3D11RenderTargetView};

mod new;
mod render_view;

/// A texture which is from the back buffer of a swapchain, used for rendering into the swapchain
pub(in crate::graphics) struct BackBufferTexture {
    /// The view for rendering into
    view: ComPtr<ID3D11RenderTargetView>,
}
