use crate::render::frame_graph::resources::FrameGraphResourceList;

impl FrameGraphResourceList {
    /// Create a new [`FrameGraphResourceList`]
    pub fn new() -> FrameGraphResourceList {
        FrameGraphResourceList {
            resources: Vec::new(),
            memory: None,
        }
    }
}
