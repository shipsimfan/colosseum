use crate::render::frame_graph::{
    FrameGraphPipelineBarrier, FrameGraphSubmissionSystem, SubmissionNode,
};

impl FrameGraphSubmissionSystem {
    /// Create a new instance of the [`FrameGraphSubmissionSystem`]
    pub fn new(
        nodes: Vec<SubmissionNode>,
        barriers: Vec<FrameGraphPipelineBarrier>,
    ) -> FrameGraphSubmissionSystem {
        FrameGraphSubmissionSystem {
            nodes,
            barriers,
            primary_command_buffer: 0,
            frame_index: 0,
            resources: 0,
        }
    }
}
