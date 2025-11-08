use win32::{
    ComPtr,
    d3d11::{
        D3D11_VIEWPORT, ID3D11DepthStencilView, ID3D11RenderTargetView, ID3D11Resource,
        ID3D11Texture2D,
    },
};

mod bind;
mod clear;
mod new;
mod unbind;

/// The objects associated directly with the swapchain that must be re-created when the swapchan is
/// resized
pub(in crate::graphics::context) struct SwapchainObjects {
    /// The viewport to render objects into
    viewport: D3D11_VIEWPORT,

    /// The buffer for writing depth information
    #[allow(unused)]
    depth_buffer: ComPtr<ID3D11Texture2D>,

    /// The view of `depth_view` for rendering
    depth_stencil_view: ComPtr<ID3D11DepthStencilView>,

    /// A reference to the swapchain's back buffer
    #[allow(unused)]
    back_buffer: ComPtr<ID3D11Resource>,

    /// The render target view of the swapchain's back buffer
    render_target_view: ComPtr<ID3D11RenderTargetView>,
}
