use crate::{
    Result,
    render::{FrameContext, FrameGraph},
};

impl FrameGraph {
    /// Execute the frame graph, rendering a frame
    pub(in crate::render::frame_graph) fn execute(&mut self, frame: FrameContext) -> Result<()> {
        // TODO: execute nodes in topological order

        frame.present()
    }
}
