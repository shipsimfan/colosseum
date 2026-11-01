use crate::{
    Result,
    render::{
        CameraDsl, GpuCamera,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage, ProceduralSkyNode,
        },
    },
};

impl ProceduralSkyNode {
    /// Add a new [`ProceduralSkyNode`] to the frame graph
    pub fn add(output: FrameGraphResourceId, builder: &mut FrameGraphBuilder) -> Result<()> {
        // Create descriptor set
        let descriptor_set =
            builder.create_descriptor_set::<CameraDsl, _>("Procedural Sky Descriptor Set");
        builder.data_access::<GpuCamera>(descriptor_set, 0);

        // Access the output as a color attachment
        builder.access(output, FrameGraphResourceUsage::ColorAttachment);

        // Add the procedural sky node to the frame graph
        builder.add_node(ProceduralSkyNode { descriptor_set }, true)
    }
}
