use crate::{
    Result,
    render::{FixedRenderObjects, RenderObjects},
};
use alexandria::{SlotMap, gpu::VulkanDevice};

impl RenderObjects {
    /// Create a new set of [`RenderObjects`]
    pub fn new(device: &VulkanDevice) -> Result<RenderObjects> {
        Ok(RenderObjects {
            fixed: FixedRenderObjects::new(device)?,
            meshes: SlotMap::new(),
            unlit_opaque_materials: SlotMap::new(),
        })
    }
}
