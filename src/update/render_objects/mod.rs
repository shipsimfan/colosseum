use crate::render::{FixedRenderObjects, GpuTransferQueue, Material, Mesh, Shader};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanFormat},
};
use std::sync::Arc;

mod allocator;

mod apply;
mod complete;
mod create;
mod get;
mod new;
mod remove;

pub(crate) use allocator::*;

/// The representations of the render objects in the update phase
pub(crate) struct UpdateRenderObjects {
    /// The device to use to create render objects
    device: VulkanDevice,

    /// The fixed render objects
    fixed_render_objects: Arc<FixedRenderObjects>,

    /// The format of the swapchain being used
    swapchain_format: VulkanFormat,

    /// The queue for transfering data to the GPU
    transfer_queue: GpuTransferQueue,

    /// The allocator to use for meshes
    mesh_allocator: GpuAllocator,

    /// The meshes that have been registered
    meshes: SlotMap<(Mesh, GpuAllocatedMemory)>,

    /// The unlit shaders that have been registered
    ///
    /// These shaders are run in a forward pass without lighting information
    unlit_shaders: SlotMap<Arc<Shader>>,

    /// The unlit opaque materials that have been registered
    ///
    /// These materials are used in a forward pass without lighting information or transparency
    unlit_opaque_materials: SlotMap<Material>,
}
