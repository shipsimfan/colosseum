use crate::render::{
    frame_graph::{FrameGraphBuilder, FrameGraphResources, FrameGraphSubmissionSystem},
    phases::FrameGraph,
};

impl<'a> FrameGraphBuilder<'a> {
    /// Register the submission system
    pub fn finish(self, resources: &mut FrameGraphResources) {
        // Insert the new resource instance into the world
        resources.clear();
        *resources = FrameGraphResources::new();

        // Register submission system
        self.commands.register_system_direct(
            FrameGraph,
            FrameGraphSubmissionSystem::new(self.nodes, self.pipeline_barriers),
        );
    }
}
