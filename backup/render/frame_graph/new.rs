use crate::render::{
    FrameGraph,
    frame_graph::{ArenaBuffer, FrameGraphResourceState},
};
use alexandria::math::Vector2u;

impl FrameGraph {
    /// Create a new empty [`FrameGraph`]
    pub fn new() -> FrameGraph {
        FrameGraph {
            structure: None,
            last_swapchain_size: Vector2u::ZERO,
            last_render_scale: 0.0,
            swapchain_final_state: FrameGraphResourceState::default(),
            transient_epoch: 0,

            nodes: Vec::new(),
            external_resources: ArenaBuffer::new(),
            transient_render_scale_info: Vec::new(),
            transient_native_scale_info: Vec::new(),

            pipeline_barrier_indices: Vec::new(),
            pipeline_barriers: Vec::new(),

            image_barriers: ArenaBuffer::new(),
            color_attachments: ArenaBuffer::new(),
        }
    }
}
