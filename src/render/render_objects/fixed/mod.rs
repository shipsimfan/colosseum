use crate::render::Pipeline;

mod get;
mod new;

/// The render objects that are created once and don't change
pub(in crate::render) struct FixedRenderObjects {
    /// The set of pipelines created for frame graph nodes that don't use materials
    pipelines: Vec<Pipeline>,
}
