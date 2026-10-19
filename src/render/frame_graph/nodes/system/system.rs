use crate::{
    Result,
    render::{
        FrameIndex, SecondaryCommandBuffer,
        frame_graph::{FrameGraphNode, FrameGraphNodeSystem},
    },
};
use alexandria::gpu::VulkanCommandBufferInheritanceInfo;
use colosseum_core::new_error;
use colosseum_ecs::{UnsafeWorldCell, World};

impl<Node: FrameGraphNode, System: 'static + crate::ecs::System> crate::ecs::System
    for FrameGraphNodeSystem<Node, System>
{
    fn initialize(&mut self, world: &mut World) {
        self.system.initialize(world);

        self.frame_index = world.get_or_reserve_resource_index::<FrameIndex>();
        self.secondary_command_buffer =
            world.get_or_reserve_resource_index::<SecondaryCommandBuffer<Node>>();
    }

    fn deferred(&self) -> bool {
        self.system.deferred()
    }

    fn run(&mut self, world: &UnsafeWorldCell) -> Result<()> {
        let frame_index = unsafe {
            world
                .get_mut()
                .get_resource_at_index_mut::<FrameIndex>(self.frame_index)
        };
        let mut command_buffer = unsafe {
            world
                .get_mut()
                .get_resource_at_index_mut::<SecondaryCommandBuffer<Node>>(
                    self.secondary_command_buffer,
                )
        };
        let command_buffer = &mut command_buffer[&frame_index];

        // Begin the command buffer
        command_buffer
            .begin(Some(VulkanCommandBufferInheritanceInfo::new(
                None, 0, None, false, 0, 0,
            )))
            .map_err(|error| {
                new_error!(
                    "unable to begin the secondary command buffer for \"{}\" - {}",
                    self.name,
                    error
                )
            })?;

        // TODO: Start a render pass if needed

        self.system.run(world)?;

        // TODO: End the render pass if needed

        // End command buffer
        command_buffer.end().map_err(|error| {
            new_error!(
                "unable to end the secondary command buffer for \"{}\" - {}",
                self.name,
                error
            )
        })?;

        Ok(())
    }

    fn apply(&mut self, world: &mut World) {
        self.system.apply(world);
    }
}
