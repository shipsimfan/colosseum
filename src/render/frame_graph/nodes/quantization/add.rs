use crate::{
    Result,
    render::{
        PostProcess, Samplers,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage, QuantizationNode,
        },
    },
};

impl QuantizationNode {
    /// Add a new [`QuantizationNode`] to the frame graph
    pub fn add(
        input: FrameGraphResourceId,
        output: FrameGraphResourceId,
        samplers: &Samplers,
        builder: &mut FrameGraphBuilder,
    ) -> Result<()> {
        let descriptor_set =
            builder.create_descriptor_set::<PostProcess, _>("Quantization Descriptor Set");

        builder.access(
            input,
            FrameGraphResourceUsage::SampledImage {
                descriptor_set,
                binding: 0,
                sampler: samplers.linear(),
            },
        );
        builder.access(output, FrameGraphResourceUsage::ColorAttachment);

        builder.add_node(
            QuantizationNode {
                descriptor_set,
                output,
            },
            true,
        )
    }
}
