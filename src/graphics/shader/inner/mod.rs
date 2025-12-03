use crate::graphics::util::{PixelShader, VertexShader};
use std::num::NonZeroU32;

mod bind;
mod id;
mod new;

/// A shader program which can be used to render
pub struct ShaderInner {
    /// The ID assigned by the graphics context which uniquely identifies this shader
    id: NonZeroU32,

    /// The vertex shader
    vertex_shader: VertexShader,

    /// The pixel shader
    pixel_shader: PixelShader,
}
