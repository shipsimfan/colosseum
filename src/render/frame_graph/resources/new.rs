use crate::render::frame_graph::FrameGraphResources;

impl FrameGraphResources {
    /// Create a new instance of [`FrameGraphResources`]
    pub fn new() -> FrameGraphResources {
        FrameGraphResources {
            external_resources: Vec::new(),
        }
    }
}
