use crate::{
    Error, Result,
    render::GpuTransferQueue,
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
        device: &VulkanDevice,
        swapchain_format: VulkanFormat,
        transfer_queue: GpuTransferQueue,
        memory_properties: Arc<VulkanAdapterMemoryProperties>,
    ) -> Result<UpdateRenderObjects> {
        let pipeline_layout = device
            .create_pipeline_layout(0, None, &[])
            .map_err(Error::new_inner)?;

        let mesh_allocator = GpuAllocator::new(
            16 * 1024 * 1024, // 16 MB
            256,              // 256 B
            4 * 1024 * 1024,  // 4 MB
            memory_properties,
            device.clone(),
        );

        Ok(UpdateRenderObjects {
            device: device.clone(),
            swapchain_format,
            pipeline_layout,
            mesh_allocator,
            meshes: SlotMap::new(),
            unlit_shaders: SlotMap::new(),
            unlit_opaque_materials: SlotMap::new(),
            transfer_queue,
        })
    }
}
