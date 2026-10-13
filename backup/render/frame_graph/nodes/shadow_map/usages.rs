use crate::render::frame_graph::{
    FrameGraphResourceId, FrameGraphResourceUsage, ShadowMapLight, ShadowMapNode,
};

impl<L: ShadowMapLight> ShadowMapNode<L> {
    /// Get the usage types for the resources that this node uses
    pub(in crate::render::frame_graph::nodes) fn usages<
        T,
        F: FnOnce(&[(FrameGraphResourceId, FrameGraphResourceUsage)]) -> T,
    >(
        &self,
        f: F,
    ) -> T {
        f(&[(
            L::RESOURCE_ID,
            FrameGraphResourceUsage::ShadowMapDepthAttachment,
        )])
    }
}
