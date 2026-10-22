use crate::render::{
    frame_graph::{FrameGraphBuilder, FrameGraphResources, FrameGraphSubmissionSystem},
    phases::FrameGraph,
};

impl<'a> FrameGraphBuilder<'a> {
    /// Register the submission system
    pub fn finish(mut self, resources: &mut FrameGraphResources) {
        let (present_barrier, native_scale_resources) = self.resources.finish();

        // Add a final pipeline barrier to prepare the swapchain image for present
        if let Some(barrier) = present_barrier {
            self.pipeline_barriers.push(barrier);
        }

        // Insert the new resource instance into the world
        resources.clear();
        *resources = FrameGraphResources::new(native_scale_resources);

        // Register submission system
        self.commands.register_system_direct(
            FrameGraph,
            FrameGraphSubmissionSystem::new(self.nodes, self.pipeline_barriers),
        );
    }
}
