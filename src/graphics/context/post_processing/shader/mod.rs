use crate::graphics::util::PixelShader;

mod bind;
mod new;

/// A shader which can be run on a post-processing step
#[derive(Clone)]
pub struct PostProcessingShader {
    /// The shader itself
    pixel_shader: PixelShader,
}
