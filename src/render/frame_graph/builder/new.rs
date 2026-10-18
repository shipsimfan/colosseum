use crate::render::{
    GraphicsDevice,
    frame_graph::{FrameGraphBuilder, FrameGraphResourceState},
};
use colosseum_ecs::Commands;

impl<'a> FrameGraphBuilder<'a> {
    /// Create a new [`FrameGraphBuilder`]
    pub fn new(commands: &'a mut Commands, device: &'a GraphicsDevice) -> FrameGraphBuilder<'a> {
        FrameGraphBuilder {
            commands,
            device,

            nodes: Vec::new(),

            current_pipeline_barriers: 0,
            pipeline_barriers: Vec::new(),

            external_resources: vec![
                FrameGraphResourceState::default(), // Swapchain image
            ],
        }
    }
}
