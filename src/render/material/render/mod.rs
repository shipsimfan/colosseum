use crate::render::Shader;
use alexandria::gpu::VulkanPipeline;
use std::sync::Arc;

mod bind;
mod new;

/// A material being used in rendering
///
/// This is the material as it exists in the render job
pub(crate) struct RenderMaterial {
    /// The pipeline this material is using
    pipeline: VulkanPipeline,

    /// The shader being used by this material
    #[allow(unused)]
    shader: Arc<Shader>,
}
