use crate::render::{
    FrameIndex, SecondaryCommandBuffer,
    frame_graph::{FrameGraphNode, SubmissionNode},
};
use alexandria::gpu::VulkanCommandBuffer;
#[cfg(debug_assertions)]
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{UnsafeWorldCell, World};
#[cfg(debug_assertions)]
use std::ffi::CString;

impl SubmissionNode {
    /// Create a new [`SubmissionNode`]
    pub fn new<Node: FrameGraphNode>(
        pipeline_barriers: usize,
        #[cfg(debug_assertions)] debug: (CString, Color3f<Srgb>),
    ) -> SubmissionNode {
        SubmissionNode {
            submit_command_buffer: submit_command_buffer::<Node>,
            get_command_buffer_index: get_command_buffer::<Node>,
            command_buffer: 0,
            pipeline_barriers,
            #[cfg(debug_assertions)]
            debug: (debug.0, debug.1.with_alpha(1.0)),
        }
    }
}

/// Get the index of the command buffer for the given node from the world
fn get_command_buffer<Node: FrameGraphNode>(world: &mut World) -> usize {
    world.get_or_reserve_resource_index::<SecondaryCommandBuffer<Node>>()
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
