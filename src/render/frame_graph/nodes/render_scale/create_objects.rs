use crate::{
    Result,
    render::{FixedRenderObjects, frame_graph::RenderScaleNode},
};
use alexandria::gpu::{VulkanDevice, VulkanFormat};

impl RenderScaleNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render) fn create_objects(
        _: &mut FixedRenderObjects,
        _: VulkanFormat,
        _: &VulkanDevice,
    ) -> Result<()> {
        Ok(())
    }
}
