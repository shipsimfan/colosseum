use crate::render::frame_graph::{FrameGraphResourceId, FrameGraphResourceUsage, ShadowMapNode};

impl ShadowMapNode {
    /// Get the usage types for the resources that this node uses
    pub(in crate::render::frame_graph::nodes) fn usages<
        T,
        F: FnOnce(&[(FrameGraphResourceId, FrameGraphResourceUsage)]) -> T,
    >(
        &self,
        f: F,
    ) -> T {
        f(&[(
            FrameGraphResourceId::SPOT_LIGHT_SHADOW_MAPS,
            FrameGraphResourceUsage::ShadowMapDepthAttachment,
        )])
    }
}
