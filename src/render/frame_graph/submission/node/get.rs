use crate::render::frame_graph::SubmissionNode;
#[cfg(debug_assertions)]
use alexandria::math::{Color4f, Srgb};
#[cfg(debug_assertions)]
use std::ffi::CStr;

impl SubmissionNode {
    /// Get the number of pipeline barriers associated with this submission node
    pub fn pipeline_barriers(&self) -> usize {
        self.pipeline_barriers
    }

    /// Get the name of the submission node
    #[cfg(debug_assertions)]
    pub fn name(&self) -> &CStr {
        &self.debug.0
    }

    /// Get the color of the submission node
    #[cfg(debug_assertions)]
    pub fn color(&self) -> Color4f<Srgb> {
        self.debug.1
    }
}
