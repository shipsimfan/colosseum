use crate::render::{Pipeline, Shader};
use std::sync::Arc;

mod get;
mod new;

/// The render objects that are created once and don't change
pub(in crate::render) struct FixedRenderObjects {
    /// A fullscreen quad shader that can be used for post-processing effects
    fullscreen_quad: Arc<Shader>,

    /// The set of pipelines created for frame graph nodes that don't use materials
    pipelines: Vec<Pipeline>,
}
