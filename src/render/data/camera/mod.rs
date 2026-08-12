use alexandria::gpu::{
    VulkanBuffer, VulkanDescriptorPool, VulkanDescriptorSet, VulkanMappedMemory,
};
use shader::CameraShaderData;

mod get;
mod new;
mod shader;

/// The data required to render a camera's view of the scene
pub(crate) struct CameraRenderData {
    /// The descriptor set containing the camera shader data
    descriptor_set: VulkanDescriptorSet,

    /// The descriptor pool containing the camera shader data
    #[allow(unused)]
    descriptor_pool: VulkanDescriptorPool,

    /// The buffer containing the camera shader data
    #[allow(unused)]
    buffer: VulkanBuffer,

    /// The memory containing the camera shader data
    shader_data: VulkanMappedMemory<CameraShaderData>,
}
