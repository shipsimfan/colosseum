use crate::{
    render::{FixedRenderObjects, GpuTransferQueue},
    update::{UpdateRenderObjects, render_objects::GpuAllocator},
};
use alexandria::{
    SlotMap,
    gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanFormat},
};
use std::sync::Arc;

impl UpdateRenderObjects {
    /// Create a new set of [`UpdateRenderObjects`]
    pub(in crate::update) fn new(
        device: VulkanDevice,
        swapchain_format: VulkanFormat,
        transfer_queue: GpuTransferQueue,
        memory_properties: Arc<VulkanAdapterMemoryProperties>,
        fixed_render_objects: Arc<FixedRenderObjects>,
    ) -> UpdateRenderObjects {
        let mesh_allocator = GpuAllocator::new(
            16 * 1024 * 1024, // 16 MB
            256,              // 256 B
            4 * 1024 * 1024,  // 4 MB
            memory_properties,
            device.clone(),
        );

        UpdateRenderObjects {
            device,
            fixed_render_objects,
            swapchain_format,
            mesh_allocator,
            meshes: SlotMap::new(),
            unlit_shaders: SlotMap::new(),
            unlit_opaque_materials: SlotMap::new(),
            transfer_queue,
        }
    }
}
