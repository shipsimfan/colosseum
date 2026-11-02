use crate::render::Shader;

mod get;
mod new;

/// Represents the fullscreen quad shader used in the rendering pipeline
pub(in crate::render) struct FullscreenQuadShader {
    /// The underlying shader used for rendering the fullscreen quad
    shader: Shader,
}
