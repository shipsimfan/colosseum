use crate::{
    Result,
    render::{
        PostProcess, SDR_FORMAT, Samplers,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind,
            FrameGraphResourceUsage, FxaaNode,
        },
    },
};
use alexandria::gpu::VulkanAttachmentLoadOp;

impl FxaaNode {
    /// Add a new [`FxaaNode`] to the frame graph
    pub fn add(
        input: FrameGraphResourceId,
        samplers: &Samplers,
        builder: &mut FrameGraphBuilder,
    ) -> Result<FrameGraphResourceId> {
        // Create descriptor set
        let descriptor_set = builder.create_descriptor_set::<PostProcess, _>("FXAA Descriptor Set");

        // Create output resource
        let output = builder.create_resource(
            "FXAA Output",
            SDR_FORMAT,
            FrameGraphResourceKind::Native,
            VulkanAttachmentLoadOp::DontCare,
        );

        // Mark resource accesses
        builder.access(
            input,
            FrameGraphResourceUsage::SampledImage {
                descriptor_set,
                binding: 0,
                sampler: samplers.linear(),
            },
        );
        builder.access(output, FrameGraphResourceUsage::ColorAttachment);

        // Add node
        builder.add_node(
            FxaaNode {
                descriptor_set,
                output,
            },
            true,
        )?;

        Ok(output)
    }
}
