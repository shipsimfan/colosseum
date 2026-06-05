use crate::render::FrameGraph;

impl FrameGraph {
    /// Create a new empty [`FrameGraph`]
    pub fn new() -> FrameGraph {
        FrameGraph {
            nodes: Vec::new(),
            image_barriers_buffer: Vec::new(),
            color_attachments_buffer: Vec::new(),
        }
    }
}
