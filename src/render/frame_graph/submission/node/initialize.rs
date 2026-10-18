use crate::render::frame_graph::SubmissionNode;
use colosseum_ecs::World;

impl SubmissionNode {
    /// Initialize the submission node
    pub(in crate::render::frame_graph) fn initialize(&mut self, world: &mut World) {
        self.command_buffer = (self.get_command_buffer_index)(world);
    }
}
