use crate::{
    Result,
    render::{Pipeline, Shader, frame_graph::RenderScaleNode},
};
use alexandria::gpu::{VulkanDevice, VulkanFormat};
use std::sync::Arc;

impl RenderScaleNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render) fn create_objects(
        _: &mut Vec<Pipeline>,
        _: &Arc<Shader>,
        _: VulkanFormat,
        _: &VulkanDevice,
    ) -> Result<()> {
        Ok(())
    }
}
