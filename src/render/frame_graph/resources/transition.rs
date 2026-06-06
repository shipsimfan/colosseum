use crate::render::frame_graph::{
    FrameGraphPipelineBarrier, FrameGraphResourceId, FrameGraphResourceState, FrameGraphResources,
};

impl<'a> FrameGraphResources<'a> {
    /// Transition a resource to a new state, returning the old state
    pub(in crate::render::frame_graph) fn transition(
        &mut self,
        resource_id: FrameGraphResourceId,
        new_state: FrameGraphResourceState,
    ) -> Option<FrameGraphPipelineBarrier> {
        if resource_id.is_external() {
            self.external[resource_id.index()].transition(resource_id, new_state)
        } else {
            todo!("transient resources are not supported yet")
        }
    }
}
