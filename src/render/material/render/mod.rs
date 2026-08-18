use crate::render::{ObjectData, Shader};
use alexandria::{
    gpu::{GpuAddress, VulkanPipeline},
    math::{Color4f, Linear},
};
use std::sync::Arc;

mod bind;
mod new;
mod set;

/// A material being used in rendering
///
/// This is the material as it exists in the render job
pub(crate) struct RenderMaterial {
    /// The color of the material
    color: Color4f<Linear>,

    /// The pipeline this material is using
    pipeline: VulkanPipeline,

    /// The shader being used by this material
    #[allow(unused)]
    shader: Arc<Shader>,
}

impl RenderMaterial {
    /// The size of the data pushed to the GPU for a material
    const DATA_SIZE: usize = std::mem::size_of::<Color4f<Linear>>();

    /// The alignment of the data pushed to the GPU for a material
    const PUSH_CONSTANT_ALIGNMENT: usize = 16;

    /// The size of the push constants for a material, including the object data
    pub(in crate::render) const PUSH_CONSTANT_SIZE: usize = (RenderMaterial::DATA_SIZE
        + std::mem::size_of::<GpuAddress<ObjectData>>())
    .next_multiple_of(RenderMaterial::PUSH_CONSTANT_ALIGNMENT);
}
