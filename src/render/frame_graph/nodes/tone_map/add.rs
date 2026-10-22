use crate::{
    Result,
    render::{
        PostProcess, Samplers,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage, ToneMapNode,
        },
    },
};
use std::marker::PhantomData;

impl<Game: crate::Game> ToneMapNode<Game> {
    /// Add a new [`ToneMapNode`] to the frame graph
    pub fn add(
        input: FrameGraphResourceId,
        output: FrameGraphResourceId,
        samplers: &Samplers,
        builder: &mut FrameGraphBuilder,
    ) -> Result<()> {
        let descriptor_set =
            builder.create_descriptor_set::<PostProcess, _>("Tone Map Descriptor Set");

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
            ToneMapNode::<Game> {
                descriptor_set,
                _marker: PhantomData,
            },
            true,
        )
    }
}
