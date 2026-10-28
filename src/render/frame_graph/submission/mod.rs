//! The system which submits the individual secondary command buffers to the primary command buffer
//! with appropriate pipeline barriers

use colosseum_ecs::WorldAccess;

mod node;
mod pipeline_barrier;

mod new;
mod system;

pub(in crate::render::frame_graph) use node::*;
pub(in crate::render::frame_graph) use pipeline_barrier::*;

/// The system which submits the individual secondary command buffers to the primary command buffer with appropriate pipeline barriers
pub(in crate::render::frame_graph) struct FrameGraphSubmissionSystem {
    /// The list of submission nodes to be processed by this system
    nodes: Vec<SubmissionNode>,

    /// The list of pipeline barriers to be applied before submitting the command buffers
    barriers: Vec<FrameGraphPipelineBarrier>,

    /// The index of the primary command buffer
    primary_command_buffer: usize,

    /// The index of the frame index resource
    frame_index: usize,

    /// The index of the resources container
    resources: usize,

    /// The access used by this system
    access: WorldAccess,
}
