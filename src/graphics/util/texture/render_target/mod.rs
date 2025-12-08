use win32::{
    ComPtr,
    d3d11::{ID3D11RenderTargetView, ID3D11ShaderResourceView},
};

mod bind;
mod clear;
mod new;
mod render_view;

/// A texture which is rendered into by a pass and then later read
pub(in crate::graphics) struct RenderTargetTexture {
    /// The view for rendering into this texture
    render_view: ComPtr<ID3D11RenderTargetView>,

    /// The view for reading from later shaders
    shader_view: ComPtr<ID3D11ShaderResourceView>,

    /// The slot to bind to when being read
    shader_slot: u32,
}
