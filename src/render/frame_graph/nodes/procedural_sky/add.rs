use crate::{
    Result,
    render::{
        CameraDsl,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage, ProceduralSkyNode,
        },
    },
};

impl ProceduralSkyNode {
    /// Add a new [`ProceduralSkyNode`] to the frame graph
    pub fn add(output: FrameGraphResourceId, builder: &mut FrameGraphBuilder) -> Result<()> {
        // Create descriptor set
        let descriptor_set = builder.get_descriptor_set::<CameraDsl>();

        // Access the output as a color attachment
        builder.access(output, FrameGraphResourceUsage::ColorAttachment);

        // Add the procedural sky node to the frame graph
        builder.add_node(ProceduralSkyNode { descriptor_set }, true)
    }
}
