use crate::render::{FrameIndex, frame_graph::SubmissionNode};
use alexandria::gpu::VulkanCommandBuffer;
use colosseum_ecs::UnsafeWorldCell;

impl SubmissionNode {
    /// Submit the secondary command buffer associated with this node to the primary command buffer
    pub fn submit(
        &self,
        world: &UnsafeWorldCell,
        primary_command_buffer: &mut VulkanCommandBuffer,
        frame_index: &FrameIndex,
    ) {
        (self.submit_command_buffer)(
            world,
            self.command_buffer,
            primary_command_buffer,
            frame_index,
        );
    }
}
