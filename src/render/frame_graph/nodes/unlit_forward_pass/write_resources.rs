use crate::render::frame_graph::{FrameGraphResourceId, UnlitForwardPassNode};

impl UnlitForwardPassNode {
    /// Get the resources that this node writes to
    pub fn write_resources<T, F: FnMut(FrameGraphResourceId) -> T>(&self, mut f: F) -> T {
        f(self.output)
    }
}
