use crate::{
    Result,
    render::{Pipeline, Shader, frame_graph::UnlitForwardRenderNode},
};
use alexandria::gpu::{VulkanDevice, VulkanFormat};
use std::sync::Arc;

impl UnlitForwardRenderNode {
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
