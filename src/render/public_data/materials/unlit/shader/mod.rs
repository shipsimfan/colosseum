use crate::render::{Pipeline, Shader};

mod new;

/// A shader for unlit materials
pub struct UnlitShader {
    /// The underlying shader object
    shader: Shader,

    /// The pipeline for the shader
    pipeline: Pipeline<UnlitShader>,
}
