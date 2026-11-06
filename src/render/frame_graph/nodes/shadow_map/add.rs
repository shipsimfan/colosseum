use crate::{
    Result,
    render::{
        ShadowMapLight,
        frame_graph::{FrameGraphBuilder, FrameGraphResourceUsage, ShadowMapNode},
    },
};
use std::marker::PhantomData;

impl<L: ShadowMapLight> ShadowMapNode<L> {
    /// Add a new [`UnlitForwardRenderNode`] to the frame graph
    pub fn add(builder: &mut FrameGraphBuilder) -> Result<()> {
        // Create descriptor set
        let descriptor_set = builder.get_descriptor_set::<L::DescriptorSetLayout>();

        // Access the output as a color attachment and the depth buffer as a depth attachment
        builder.access(L::SHADOW_MAP, FrameGraphResourceUsage::DepthAttachment);

        // Add the unlit forward render node to the frame graph
        builder.add_node(
            ShadowMapNode::<L> {
                descriptor_set,
                _light: PhantomData,
            },
            true,
        )
    }
}
