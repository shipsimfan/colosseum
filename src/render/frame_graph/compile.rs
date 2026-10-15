use crate::render::{FrameGraph, frame_graph::SolidColorSkyNode, phases};
use colosseum_ecs::World;

impl FrameGraph {
    /// Compile the frame graph, preparing it for execution
    pub fn compile(&self, world: &mut World) {
        world.register_system(phases::FrameGraph, SolidColorSkyNode::system);
    }
}
