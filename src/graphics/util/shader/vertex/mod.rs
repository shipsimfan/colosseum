use win32::{
    ComPtr,
    d3d11::{ID3D11InputLayout, ID3D11VertexShader},
};

mod bind;
mod new;

/// A shader program which is run on vertices passed to a draw call
pub(in crate::graphics) struct VertexShader {
    /// The pointer to the shader itself
    shader: ComPtr<ID3D11VertexShader>,

    /// The input layout used by the shader
    input_layout: ComPtr<ID3D11InputLayout>,
}
