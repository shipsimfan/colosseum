use crate::{
    Result,
    render::{
        DescriptorSetLayouts,
        frame_graph::{FrameGraphBuilder, FrameGraphResources, FrameGraphSubmissionSystem},
        phases::FrameGraph,
    },
};

impl<'a> FrameGraphBuilder<'a> {
    /// Register the submission system
    pub fn finish(
        mut self,
        resources: &mut FrameGraphResources,
        descriptor_set_layouts: &DescriptorSetLayouts,
    ) -> Result<()> {
        let (present_barrier, native_scale_resources, render_scale_resources) =
            self.resources.finish();

        // Add a final pipeline barrier to prepare the swapchain image for present
        if let Some(barrier) = present_barrier {
            self.pipeline_barriers.push(barrier);
        }

        // Insert the new resource instance into the world
        resources.clear();
        *resources = FrameGraphResources::new(
            native_scale_resources,
            render_scale_resources,
            self.descriptors,
            self.descriptor_sets,
            descriptor_set_layouts,
            self.device,
        )?;

        // Register submission system
        self.commands.register_system(
            FrameGraph,
            FrameGraphSubmissionSystem::new(self.nodes, self.pipeline_barriers),
        );

        Ok(())
    }
}
