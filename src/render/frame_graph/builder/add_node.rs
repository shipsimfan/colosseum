use crate::{
    Result,
    render::{
        SecondaryCommandBuffer,
        frame_graph::{FrameGraphBuilder, FrameGraphNode, SubmissionNode},
        phases::FrameGraph,
    },
};

impl<'a> FrameGraphBuilder<'a> {
    /// Add a new node to the frame graph
    pub fn add_node<Node: FrameGraphNode>(&mut self, node: Node) -> Result<()> {
        let name = node.name().into();

        // Create secondary command buffer
        self.commands
            .insert_resource(SecondaryCommandBuffer::<Node>::new(&name, self.device)?);

        // Wrap and register the system
        self.commands
            .register_system_direct(FrameGraph, node.system());

        // Insert submission node
        self.nodes.push(SubmissionNode::new::<Node>(
            self.current_pipeline_barriers,
            #[cfg(debug_assertions)]
            (node.name().into(), node.color().into()),
        ));

        self.current_pipeline_barriers = 0;

        Ok(())
    }
}
