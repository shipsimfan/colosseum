use crate::{
    Result,
    render::{
        FrameIndex, SecondaryCommandBuffer,
        frame_graph::{FrameGraphNode, FrameGraphNodeSystem},
    },
};
use alexandria::gpu::VulkanCommandBufferInheritanceInfo;
use colosseum_core::new_error;
use colosseum_ecs::{IntoSystem, UnsafeWorldCell, World};

impl<Node: FrameGraphNode, System: 'static + crate::ecs::System> crate::ecs::System
    for FrameGraphNodeSystem<Node, System>
{
    fn initialize(&mut self, world: &mut World) {
        self.system.initialize(world);

        self.frame_index = world.get_or_reserve_permanent_resource_index::<FrameIndex>();
        self.secondary_command_buffer =
            world.get_or_reserve_resource_index::<SecondaryCommandBuffer<Node>>();

        if let Some(render_pass) = &mut self.render_pass {
            render_pass.initialize(world);
        }
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
        let mut cmd_buffer = unsafe {
            world
                .get_mut()
                .get_resource_at_index_mut::<SecondaryCommandBuffer<Node>>(
                    self.secondary_command_buffer,
                )
        };
        let cmd_buffer = &mut cmd_buffer[&frame_index];

        // Begin the command buffer
        cmd_buffer
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

        // Start a render pass if needed
        if let Some(render_pass) = &self.render_pass {
            render_pass.start(cmd_buffer, world);
        }

        self.system.run(world)?;

        // End the render pass if needed
        if self.render_pass.is_some() {
            cmd_buffer.cmd_end_rendering();
        }

        // End command buffer
        cmd_buffer.end().map_err(|error| {
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

impl<Node: FrameGraphNode, System: 'static + crate::ecs::System> IntoSystem<UnsafeWorldCell>
    for FrameGraphNodeSystem<Node, System>
{
    type System = Self;

    fn into_system(self) -> Self::System {
        self
    }
}
