use crate::render::frame_graph::{
    FrameGraphExternalResource, FrameGraphPipelineBarrier, FrameGraphResourceId,
    FrameGraphResourceState,
};

impl<'a> FrameGraphExternalResource<'a> {
    /// Transition the resource to a new state, returning a pipeline barrier if needed
    pub(in crate::render::frame_graph::resources) fn transition(
        &mut self,
        resource: FrameGraphResourceId,
        new_state: FrameGraphResourceState,
    ) -> Option<FrameGraphPipelineBarrier> {
        self.state.transition(resource, new_state)
    }
}
