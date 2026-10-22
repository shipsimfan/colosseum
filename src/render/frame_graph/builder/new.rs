use crate::render::{
    GraphicsDevice,
    frame_graph::{FrameGraphBuilder, builder::FrameGraphBuilderResources},
};
use colosseum_ecs::Commands;

impl<'a> FrameGraphBuilder<'a> {
    /// Create a new [`FrameGraphBuilder`]
    pub fn new(commands: &'a mut Commands, device: &'a GraphicsDevice) -> FrameGraphBuilder<'a> {
        FrameGraphBuilder {
            commands,
            device,

            nodes: Vec::new(),
            pipeline_barriers: Vec::new(),
            resources: FrameGraphBuilderResources::new(),

            current_pipeline_barriers: 0,
            color_attachment: None,
        }
    }
}
