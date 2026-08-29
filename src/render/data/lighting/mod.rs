use alexandria::gpu::{VulkanBuffer, VulkanDescriptorSet, VulkanMappedMemory};
use buffer::*;
use metadata::*;

mod buffer;
mod directional;
mod metadata;
mod point;

mod add;
mod get;
mod new;
mod reserve;
mod reset;
mod set;

pub(crate) use directional::*;
pub(crate) use point::*;

/// The data about lighting for a given frame
pub(crate) struct LightingData {
    /// The descriptor set containing all the lighting data
    descriptor_set: VulkanDescriptorSet,

    /// The buffer containing the lighting metadata
    #[allow(unused)]
    metadata_buffer: VulkanBuffer,

    /// The memory containing the lighting metadata
    metadata: VulkanMappedMemory<LightingMetadata>,

    /// The buffer containing the directional light data
    directional_lights: LightingDataBuffer<RenderDirectionalLight>,

    /// The buffer containing the point light data
    point_lights: LightingDataBuffer<RenderPointLight>,
}
