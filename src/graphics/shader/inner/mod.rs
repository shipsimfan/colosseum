use std::num::NonZeroU32;
use win32::{
    ComPtr,
    d3d11::{ID3D11InputLayout, ID3D11PixelShader, ID3D11VertexShader},
};

mod bind;
mod id;
mod new;

/// A shader program which can be used to render
pub struct ShaderInner {
    /// The ID assigned by the graphics context which uniquely identifies this shader
    id: NonZeroU32,

    /// The vertex shader
    vertex_shader: ComPtr<ID3D11VertexShader>,

    /// The pixel shader
    pixel_shader: ComPtr<ID3D11PixelShader>,

    /// The input layout describing vertices
    input_layout: ComPtr<ID3D11InputLayout>,
}
