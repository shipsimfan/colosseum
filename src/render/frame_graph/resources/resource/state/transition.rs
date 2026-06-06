use crate::render::frame_graph::{
    FrameGraphPipelineBarrier, FrameGraphResourceId, FrameGraphResourceState,
};

impl FrameGraphResourceState {
    /// Transition the resource to a new state, returning the old state
    pub(in crate::render::frame_graph::resources) fn transition(
        &mut self,
        resource: FrameGraphResourceId,
        new_state: FrameGraphResourceState,
    ) -> Option<FrameGraphPipelineBarrier> {
        if let Some(barrier) =
            FrameGraphPipelineBarrier::new(resource, self.clone(), new_state.clone())
        {
            *self = new_state;
            return Some(barrier);
        }

        None
    }
}
