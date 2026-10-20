use crate::render::frame_graph::{FrameGraphResources, nodes::system::FrameGraphRenderPass};
use colosseum_ecs::World;

impl FrameGraphRenderPass {
    /// Initialize the render pass with the given world context
    pub fn initialize(&mut self, world: &mut World) {
        self.resources = world.get_or_reserve_resource_index::<FrameGraphResources>();
    }
}
