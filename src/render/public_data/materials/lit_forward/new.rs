use crate::render::{LitForwardMaterial, LitForwardShader};
use alexandria::math::{Color3f, Srgb};
use std::sync::Arc;

impl LitForwardMaterial {
    /// Create a new [`LitForwardMaterial`]
    pub fn new<C: Into<Color3f<Srgb>>>(
        color: C,
        specular_strength: f32,
        shininess: f32,
        shader: Arc<LitForwardShader>,
    ) -> LitForwardMaterial {
        LitForwardMaterial {
            color: color.into().into_linear(),
            specular_strength,
            shininess,
            shader,
        }
    }
}
