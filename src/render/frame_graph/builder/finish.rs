use crate::render::{
    FrameIndex, RenderCommandBuffer,
    frame_graph::{FrameGraphBuilder, FrameGraphResources, submission_system},
    phases::FrameGraph,
};
use colosseum_ecs::{Res, ResMut};

impl<'a> FrameGraphBuilder<'a> {
    /// Register the submission system
    pub fn finish(self, resources: &mut FrameGraphResources) {
        // Insert the new resource instance into the world
        resources.clear();
        *resources = FrameGraphResources::new();

        // Register submission system
        self.commands.register_system(
            FrameGraph,
            move |mut command_buffer: ResMut<RenderCommandBuffer>,
                  resources: Res<FrameGraphResources>,
                  frame_index: Res<FrameIndex>| {
                submission_system(
                    &self.nodes,
                    &self.pipeline_barriers,
                    &resources,
                    &frame_index,
                    &mut command_buffer,
                )
            },
        );
    }
}
