use crate::render::frame_graph::{
    FrameGraphPipelineBarrier, FrameGraphResourceId, FrameGraphResourceState,
    builder::FrameGraphBuilderResource,
};

impl<'a> FrameGraphBuilderResource<'a> {
    /// Transition the resource to a new state, returning a pipeline barrier if needed
    pub fn transition(
        &mut self,
        resource: FrameGraphResourceId,
        new_state: FrameGraphResourceState,
    ) -> Option<FrameGraphPipelineBarrier> {
        match self {
            FrameGraphBuilderResource::External(external) => {
                external.transition(resource, new_state)
            }
            FrameGraphBuilderResource::Transient(transient) => {
                transient.transition(resource, new_state)
            }
        }
    }
}
