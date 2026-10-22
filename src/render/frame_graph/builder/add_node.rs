use crate::{
    Result,
    render::{
        SecondaryCommandBuffer,
        frame_graph::{FrameGraphBuilder, FrameGraphNode, FrameGraphNodeSystem, SubmissionNode},
        phases::FrameGraph,
    },
};
#[cfg(debug_assertions)]
use std::{ffi::CString, str::FromStr};

impl<'a> FrameGraphBuilder<'a> {
    /// Add a new node to the frame graph
    pub fn add_node<Node: FrameGraphNode>(&mut self, node: Node, resource: bool) -> Result<()> {
        let name = node.name().into();
        #[cfg(debug_assertions)]
        let c_name = CString::from_str(&name).unwrap();

        // Create secondary command buffer
        self.commands
            .insert_resource(SecondaryCommandBuffer::<Node>::new(&name, self.device)?);

        // Wrap and register the system
        self.commands
            .register_system_direct::<_, FrameGraphNodeSystem<Node, _>>(
                FrameGraph,
                FrameGraphNodeSystem::new(name, node.system(), self.color_attachment.take()),
            );

        // Insert submission node
        self.nodes.push(SubmissionNode::new::<Node>(
            self.current_pipeline_barriers,
            #[cfg(debug_assertions)]
            (c_name, node.color().into()),
        ));

        if resource {
            self.commands.insert_resource(node);
        }

        self.current_pipeline_barriers = 0;

        Ok(())
    }
}
