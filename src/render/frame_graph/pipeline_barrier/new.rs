use crate::render::frame_graph::{
    FrameGraphPipelineBarrier, FrameGraphResourceId, FrameGraphResourceState,
};

impl FrameGraphPipelineBarrier {
    /// Create a new [`FrameGraphPipelineBarrier`]
    pub fn new(
        resource: FrameGraphResourceId,
        old_state: FrameGraphResourceState,
        new_state: FrameGraphResourceState,
    ) -> Option<FrameGraphPipelineBarrier> {
        if old_state == new_state {
            return None;
        }

        Some(FrameGraphPipelineBarrier {
            resource,
            old_state,
            new_state,
        })
    }
}
