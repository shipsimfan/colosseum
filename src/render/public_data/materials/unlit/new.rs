use crate::render::{UnlitMaterial, UnlitShader};
use alexandria::math::{Color3f, Srgb};
use std::sync::Arc;

impl UnlitMaterial {
    /// Create a new [`UnlitMaterial`]
    pub fn new<C: Into<Color3f<Srgb>>>(color: C, shader: Arc<UnlitShader>) -> UnlitMaterial {
        UnlitMaterial {
            color: color.into().into_linear(),
            shader,
        }
    }
}
