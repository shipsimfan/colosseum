use crate::{
    Result,
    render::{
        PostProcess, SDR_FORMAT, Samplers,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind,
            FrameGraphResourceUsage, ToneMapNode,
        },
    },
};
use alexandria::gpu::VulkanAttachmentLoadOp;
use std::marker::PhantomData;

impl<Game: crate::Game> ToneMapNode<Game> {
    /// Add a new [`ToneMapNode`] to the frame graph
    pub fn add(
        input: FrameGraphResourceId,
        samplers: &Samplers,
        builder: &mut FrameGraphBuilder,
    ) -> Result<FrameGraphResourceId> {
        // Create descriptor set
        let descriptor_set =
            builder.create_descriptor_set::<PostProcess, _>("Tone Map Descriptor Set");

        // Create output
        let output = builder.create_resource(
            "Tone Map Output",
            SDR_FORMAT,
            FrameGraphResourceKind::Render,
            VulkanAttachmentLoadOp::DontCare,
        );

        // Mark accesses
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
            ToneMapNode::<Game> {
                descriptor_set,
                _marker: PhantomData,
            },
            true,
        )?;

        Ok(output)
    }
}
