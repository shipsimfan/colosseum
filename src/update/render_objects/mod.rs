use crate::render::{Material, Shader};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanFormat, VulkanPipelineLayout},
};
use std::sync::Arc;

mod create;
mod new;
mod remove;

/// The representations of the render objects in the update phase
pub(crate) struct UpdateRenderObjects {
    /// The device to use to create render objects
    device: VulkanDevice,

    /// The format of the swapchain being used
    swapchain_format: VulkanFormat,

    /// The pipeline layout used by materials
    pipeline_layout: VulkanPipelineLayout,

    /// The unlit shaders that have been registered
    ///
    /// These shaders are run in a forward pass without lighting information
    unlit_shaders: SlotMap<Arc<Shader>>,

    /// The unlit opaque materials that have been registered
    ///
    /// These materials are used in a forward pass without lighting information or transparency
    unlit_opaque_materials: SlotMap<Material>,
}
