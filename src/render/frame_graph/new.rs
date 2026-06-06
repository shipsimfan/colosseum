use crate::render::{FrameGraph, frame_graph::ArenaBuffer};

impl FrameGraph {
    /// Create a new empty [`FrameGraph`]
    pub fn new() -> FrameGraph {
        FrameGraph {
            structure: None,
            nodes: Vec::new(),
            external_resources: ArenaBuffer::new(),
            pipeline_barrier_indices: Vec::new(),
            pipeline_barriers: Vec::new(),
            image_barriers: ArenaBuffer::new(),
            color_attachments: ArenaBuffer::new(),
        }
    }
}
