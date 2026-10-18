use crate::render::FrameIndex;
use alexandria::{
    gpu::VulkanCommandBuffer,
    math::{Color3f, Srgb},
};
use colosseum_ecs::{UnsafeWorldCell, World};
use std::borrow::Cow;

mod add_pipeline_barriers;
mod get;
mod initialize;
mod new;
mod submit;

/// A single node that must be submitted to the primary command buffer
pub(in crate::render::frame_graph) struct SubmissionNode {
    /// The function to submit a secondary command buffer to the primary command buffer
    submit_command_buffer: fn(&UnsafeWorldCell, usize, &mut VulkanCommandBuffer, &FrameIndex),

    /// The function to get the index of the command buffer for this node from the world
    get_command_buffer_index: fn(&mut World) -> usize,

    /// The index of the command buffer for this node
    command_buffer: usize,

    /// The number of pipeline barriers associated with this submission node
    pipeline_barriers: usize,

    /// The name and color to use for the debug label of this node
    #[cfg(debug_assertions)]
    debug: (Cow<'static, str>, Color3f<Srgb>),
}
