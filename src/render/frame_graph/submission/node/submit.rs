use crate::render::frame_graph::{FrameGraphNode, SubmissionNode};
use alexandria::gpu::VulkanCommandBuffer;

impl SubmissionNode {
    /// Submit the secondary command buffer associated with this node to the primary command buffer
    pub fn submit(&self, primary_command_buffer: &mut VulkanCommandBuffer) {
        (self.submit_command_buffer)(
            &unsafe { colosseum_ecs::UnsafeWorldCell::new() },
            self.command_buffer,
            &mut primary_command_buffer[&crate::render::FrameIndex::default()],
            &crate::render::FrameIndex::default(),
        );
    }
}
