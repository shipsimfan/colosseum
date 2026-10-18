use crate::{
    Result,
    render::{
        FrameIndex, RenderCommandBuffer,
        frame_graph::{FrameGraphResources, FrameGraphSubmissionSystem},
    },
};
use colosseum_ecs::{System, UnsafeWorldCell, World};

impl System for FrameGraphSubmissionSystem {
    fn initialize(&mut self, world: &mut World) {
        self.primary_command_buffer = world.get_or_reserve_resource_index::<RenderCommandBuffer>();
        self.frame_index = world.get_or_reserve_resource_index::<FrameIndex>();
        self.resources = world.get_or_reserve_resource_index::<FrameGraphResources>();

        for node in &mut self.nodes {
            node.initialize(world);
        }
    }

    fn run(&mut self, world: &UnsafeWorldCell) -> Result<()> {
        let frame_index = unsafe {
            world
                .get()
                .get_resource_at_index::<FrameIndex>(self.frame_index)
                .clone()
        };
        let primary_command_buffer = unsafe {
            &mut world
                .get_mut()
                .get_resource_at_index_mut::<RenderCommandBuffer>(self.primary_command_buffer)
                [&frame_index]
        };
        let resources = unsafe {
            world
                .get_mut()
                .get_resource_at_index_mut::<FrameGraphResources>(self.resources)
        };

        let mut barriers = self.barriers.as_slice();
        for node in &self.nodes {
            // Add pipeline barriers
            node.add_pipeline_barriers(primary_command_buffer, barriers, &resources);

            // Submit the secondary command buffer to the primary command buffer
            node.submit(world, primary_command_buffer, &frame_index);

            barriers = &barriers[node.pipeline_barriers()..];
        }

        Ok(())
    }
}
