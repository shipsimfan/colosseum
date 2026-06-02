use crate::render::frame_graph::resources::pool::FrameGraphResourcesPool;

impl FrameGraphResourcesPool {
    /// Create a new [`FrameGraphResourcesPool`]
    pub fn new() -> FrameGraphResourcesPool {
        FrameGraphResourcesPool {
            transient: Vec::new(),
            external: Vec::new(),
        }
    }
}
