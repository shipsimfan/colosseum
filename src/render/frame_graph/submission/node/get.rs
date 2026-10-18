use crate::render::frame_graph::SubmissionNode;

impl SubmissionNode {
    /// Get the number of pipeline barriers associated with this submission node
    pub fn pipeline_barriers(&self) -> usize {
        self.pipeline_barriers
    }
}
