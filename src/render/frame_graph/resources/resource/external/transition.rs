use crate::render::frame_graph::{
    FrameGraphExternalResource, FrameGraphPipelineBarrier, FrameGraphResourceId,
    FrameGraphResourceState,
};

impl<'a> FrameGraphExternalResource<'a> {
    /// Transition a resource to a new state, returning the old state
    pub(in crate::render::frame_graph::resources) fn transition(
        &mut self,
        resource_id: FrameGraphResourceId,
        new_state: FrameGraphResourceState,
    ) -> Option<FrameGraphPipelineBarrier> {
        self.state.transition(resource_id, new_state)
    }
}
