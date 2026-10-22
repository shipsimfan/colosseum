use crate::render::frame_graph::{
    FrameGraphExternalResourceInfo, FrameGraphPipelineBarrier, FrameGraphResourceId,
    FrameGraphResourceState,
};

impl FrameGraphExternalResourceInfo {
    /// Transition the resource to a new state, returning a pipeline barrier if needed
    pub fn transition(
        &mut self,
        resource: FrameGraphResourceId,
        new_state: FrameGraphResourceState,
    ) -> Option<FrameGraphPipelineBarrier> {
        self.state.transition(resource, new_state)
    }
}
