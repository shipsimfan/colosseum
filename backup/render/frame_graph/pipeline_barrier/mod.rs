use crate::render::frame_graph::{FrameGraphResourceId, FrameGraphResourceState};

mod barrier;
mod new;

/// A description of a pipeline barrier
pub(in crate::render::frame_graph) struct FrameGraphPipelineBarrier {
    /// The resource that the barrier is for
    resource: FrameGraphResourceId,

    /// The old state of the resource
    old_state: FrameGraphResourceState,

    /// The new state of the resource
    new_state: FrameGraphResourceState,
}
