use crate::render::{
    FrameIndex, SecondaryCommandBuffer,
    frame_graph::{FrameGraphNode, SubmissionNode},
};
use alexandria::gpu::VulkanCommandBuffer;
#[cfg(debug_assertions)]
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::UnsafeWorldCell;
#[cfg(debug_assertions)]
use std::borrow::Cow;

impl SubmissionNode {
    /// Create a new [`SubmissionNode`]
    pub fn new<Node: FrameGraphNode>(
        pipeline_barriers: usize,
        #[cfg(debug_assertions)] debug: (Cow<'static, str>, Color3f<Srgb>),
    ) -> SubmissionNode {
        SubmissionNode {
            submit_command_buffer: submit_command_buffer::<Node>,
            command_buffer: 0,
            pipeline_barriers,
            #[cfg(debug_assertions)]
            debug,
        }
    }
}

/// Submit the secondary command buffer associated with the given node to the primary command buffer
fn submit_command_buffer<Node: FrameGraphNode>(
    world: &UnsafeWorldCell,
    index: usize,
    primary_command_buffer: &mut VulkanCommandBuffer,
    frame_index: &FrameIndex,
) {
    let mut secondary_command_buffer = unsafe {
        world
            .get_mut()
            .get_resource_at_index_mut::<SecondaryCommandBuffer<Node>>(index)
    };

    primary_command_buffer
        .cmd_execute_commands(&mut secondary_command_buffer.as_mut()[&frame_index]);
}
