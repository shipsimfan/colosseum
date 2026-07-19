use crate::render::{GpuTransferQueue, Material, Mesh, Shader};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanDeviceMemory, VulkanFormat, VulkanPipelineLayout},
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

    /// The queue for transfering data to the GPU
    transfer_queue: GpuTransferQueue,

    /// The memory type index for device local buffers
    device_local_memory_type: usize,

    /// The meshes that have been registered
    meshes: SlotMap<(Arc<Mesh>, Arc<VulkanDeviceMemory>, Arc<VulkanDeviceMemory>)>,

    /// The unlit shaders that have been registered
    ///
    /// These shaders are run in a forward pass without lighting information
    unlit_shaders: SlotMap<Arc<Shader>>,

    /// The unlit opaque materials that have been registered
    ///
    /// These materials are used in a forward pass without lighting information or transparency
    unlit_opaque_materials: SlotMap<Material>,
}
