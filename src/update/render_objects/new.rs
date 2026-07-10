use crate::{Error, Result, update::UpdateRenderObjects};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanFormat},
};

impl UpdateRenderObjects {
    /// Create a new set of [`UpdateRenderObjects`]
    pub(in crate::update) fn new(
        device: &VulkanDevice,
        swapchain_format: VulkanFormat,
    ) -> Result<UpdateRenderObjects> {
        let pipeline_layout = device
            .create_pipeline_layout(0, None, &[])
            .map_err(Error::new_inner)?;

        Ok(UpdateRenderObjects {
            device: device.clone(),
            swapchain_format,
            pipeline_layout,
            unlit_shaders: SlotMap::new(),
            unlit_opaque_materials: SlotMap::new(),
        })
    }
}
