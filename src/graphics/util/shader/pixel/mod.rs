use win32::{ComPtr, d3d11::ID3D11PixelShader};

mod bind;
mod new;

/// A shader program which is run on each visible pixel of an object
pub(in crate::graphics) struct PixelShader {
    /// The shader itself
    shader: ComPtr<ID3D11PixelShader>,
}
