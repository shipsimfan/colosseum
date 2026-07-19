use crate::{Error, Result, render::GpuTransferQueue, update::UpdateRenderObjects};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanFormat},
};

impl UpdateRenderObjects {
    /// Create a new set of [`UpdateRenderObjects`]
    pub(in crate::update) fn new(
        device: &VulkanDevice,
        swapchain_format: VulkanFormat,
        transfer_queue: GpuTransferQueue,
        device_local_memory_type: usize,
    ) -> Result<UpdateRenderObjects> {
        let pipeline_layout = device
            .create_pipeline_layout(0, None, &[])
            .map_err(Error::new_inner)?;

        Ok(UpdateRenderObjects {
            device: device.clone(),
            swapchain_format,
            pipeline_layout,
            meshes: SlotMap::new(),
            unlit_shaders: SlotMap::new(),
            unlit_opaque_materials: SlotMap::new(),
            transfer_queue,
            device_local_memory_type,
        })
    }
}
