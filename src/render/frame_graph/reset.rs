use crate::render::FrameGraph;

impl FrameGraph {
    /// Reset the frame graph, clearing all nodes and allowing it to be built again
    pub(in crate::render::frame_graph) fn reset(&mut self) {
        self.nodes.clear();
    }
}
