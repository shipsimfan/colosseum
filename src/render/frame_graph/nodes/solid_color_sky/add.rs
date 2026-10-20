use crate::{
    Result,
    render::frame_graph::{
        FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage, SolidColorSkyNode,
    },
};

impl SolidColorSkyNode {
    /// Add a new [`SolidColorSkyNode`] to the frame graph
    pub fn add(output: FrameGraphResourceId, builder: &mut FrameGraphBuilder) -> Result<()> {
        builder.access(output, FrameGraphResourceUsage::ColorAttachment);

        builder.add_node(SolidColorSkyNode)
    }
}
