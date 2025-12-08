use crate::graphics::util::{PixelShader, VertexShader};

mod bind;
mod new;

/// A shader program which can be used to render objects
pub(in crate::graphics::managed_objects::material) struct MaterialShader {
    /// The vertex shader
    vertex_shader: VertexShader,

    /// The pixel shader
    pixel_shader: PixelShader,
}
