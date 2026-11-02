use alexandria::gpu::{VulkanPipeline, VulkanPipelineLayout};
use std::marker::PhantomData;

mod bind;
mod get;
mod new;
mod new_post_process;

/// A pipeline and layout that can be used for rendering objects
pub(in crate::render) struct Pipeline<Kind> {
    /// The pipeline itself
    pipeline: VulkanPipeline,

    /// The pipeline layout used to create the pipeline
    layout: VulkanPipelineLayout,

    /// A marker to differentiate different pipelines in the resource pool
    _kind: PhantomData<fn() -> Kind>,
}
