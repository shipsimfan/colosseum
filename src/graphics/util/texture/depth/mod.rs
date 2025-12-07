use win32::{ComPtr, d3d11::ID3D11DepthStencilView};

mod clear;
mod depth_view;
mod new;

/// A texture which can be used as the depth buffer for a render pass
pub(in crate::graphics) struct DepthTexture {
    /// The view for rendering depth information into
    view: ComPtr<ID3D11DepthStencilView>,
}
