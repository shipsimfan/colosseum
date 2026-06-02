use crate::render::{FrameGraph, frame_graph::FrameGraphNode};

impl FrameGraph {
    /// Insert a new node into the frame graph
    pub(in crate::render::frame_graph) fn add_node<N: Into<FrameGraphNode>>(&mut self, node: N) {
        self.nodes.push(node.into());
    }
}
