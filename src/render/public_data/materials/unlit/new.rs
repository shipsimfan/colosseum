use crate::render::{UnlitMaterial, UnlitShader};
use alexandria::math::{Color4f, Srgb};
use std::sync::Arc;

impl UnlitMaterial {
    /// Create a new [`UnlitMaterial`]
    pub fn new<C: Into<Color4f<Srgb>>>(color: C, shader: Arc<UnlitShader>) -> UnlitMaterial {
        UnlitMaterial {
            color: color.into().into_linear(),
            shader,
        }
    }
}
