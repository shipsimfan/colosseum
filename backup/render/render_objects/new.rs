use crate::{
    Result,
    render::{FixedRenderObjects, RenderObjects},
};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanFormat},
};

impl RenderObjects {
    /// Create a new set of [`RenderObjects`]
    pub fn new(swapchain_format: VulkanFormat, device: &VulkanDevice) -> Result<RenderObjects> {
        Ok(RenderObjects {
            fixed: FixedRenderObjects::new(swapchain_format, device)?,
            meshes: SlotMap::new(),
            unlit_opaque_materials: SlotMap::new(),
            lit_opaque_materials: SlotMap::new(),
        })
    }
}
