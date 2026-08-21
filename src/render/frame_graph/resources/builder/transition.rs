use crate::render::frame_graph::{
    FrameGraphPipelineBarrier, FrameGraphResourceBuilder, FrameGraphResourceId,
    FrameGraphResourceState,
};

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Transition a resource to a new state, returning a pipeline barrier if needed
    pub fn transition(
        &mut self,
        id: FrameGraphResourceId,
        new_state: FrameGraphResourceState,
    ) -> Option<FrameGraphPipelineBarrier> {
        if id.is_external() {
            self.external[id.index()].transition(id, new_state)
        } else if id.is_transient_render_scale() {
            self.transient_render_scale[id.index()].transition(id, new_state)
        } else if id.is_transient_native_scale() {
            todo!("transient native scale resources are not yet implemented")
        } else {
            todo!("transient static resources are not yet implemented")
        }
    }
}
