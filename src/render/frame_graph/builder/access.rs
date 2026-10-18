use crate::render::frame_graph::{
    FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage,
};

impl<'a> FrameGraphBuilder<'a> {
    /// Specify a requested access for a resource within the frame graph
    pub fn access(&mut self, resource: FrameGraphResourceId, usage: FrameGraphResourceUsage) {
        if resource.is_external() {
            if let Some(barrier) =
                self.external_resources[resource.index()].transition(resource, usage.state())
            {
                self.pipeline_barriers.push(barrier);
                self.current_pipeline_barriers += 1;
            }
            return;
        }

        panic!("Access to non-external resources is not yet implemented");
    }
}
