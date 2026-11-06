use crate::{
    Result,
    render::{
        CameraDsl, LightsDsl, RenderableDsl, Samplers,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage, LitForwardRenderNode,
        },
    },
};

impl LitForwardRenderNode {
    /// Add a new [`LitForwardRenderNode`] to the frame graph
    pub fn add(
        output: FrameGraphResourceId,
        depth_buffer: FrameGraphResourceId,
        samplers: &Samplers,
        builder: &mut FrameGraphBuilder,
    ) -> Result<()> {
        // Create descriptor set
        let camera_descriptor_set = builder.get_descriptor_set::<CameraDsl>();
        let renderable_descriptor_set = builder.get_descriptor_set::<RenderableDsl>();
        let light_descriptor_set = builder.get_descriptor_set::<LightsDsl>();

        // Access the output as a color attachment and the depth buffer as a depth attachment
        builder.access(output, FrameGraphResourceUsage::ColorAttachment);
        builder.access(depth_buffer, FrameGraphResourceUsage::DepthAttachment);
        builder.access(
            FrameGraphResourceId::DIRECTIONAL_LIGHT_SHADOW_MAP,
            FrameGraphResourceUsage::SampledImage {
                descriptor_set: light_descriptor_set,
                binding: 3,
                sampler: samplers.pcf(),
            },
        );
        builder.access(
            FrameGraphResourceId::POINT_LIGHT_SHADOW_MAP,
            FrameGraphResourceUsage::SampledImage {
                descriptor_set: light_descriptor_set,
                binding: 6,
                sampler: samplers.pcf(),
            },
        );
        builder.access(
            FrameGraphResourceId::SPOT_LIGHT_SHADOW_MAP,
            FrameGraphResourceUsage::SampledImage {
                descriptor_set: light_descriptor_set,
                binding: 9,
                sampler: samplers.pcf(),
            },
        );

        // Add the unlit forward render node to the frame graph
        builder.add_node(
            LitForwardRenderNode {
                camera_descriptor_set,
                renderable_descriptor_set,
                light_descriptor_set,
            },
            true,
        )
    }
}
