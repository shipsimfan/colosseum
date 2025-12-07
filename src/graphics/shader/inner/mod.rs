use crate::graphics::util::{PixelShader, VertexShader};

mod bind;
mod new;

/// A shader program which can be used to render
pub struct ShaderInner {
    /// The vertex shader
    vertex_shader: VertexShader,

    /// The pixel shader
    pixel_shader: PixelShader,
}
