use crate::render::frame_graph::{
    FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage,
};

impl<'a> FrameGraphBuilder<'a> {
    /// Specify a requested access for a resource within the frame graph
    pub fn access(&mut self, resource: FrameGraphResourceId, usage: FrameGraphResourceUsage) {
        // Update resources for the render pass for this node
        match usage {
            FrameGraphResourceUsage::ColorAttachment => {
                if self.color_attachment.is_some() {
                    panic!("multiple color attachments defined for a node");
                }

                self.color_attachment = Some(resource);

                if resource.is_transient_native_scale() {
                    self.native_scale_resources[resource.index()].set_color();
                }
            }
            _ => {}
        }

        // Transition the resource and a pipeline barrier if necessary
        let barrier = if resource.is_external() {
            self.external_resources[resource.index()].transition(resource, usage.state())
        } else if resource.is_transient_native_scale() {
            self.native_scale_resources[resource.index()].transition(resource, usage.state())
        } else {
            panic!("Access to non-external resources is not yet implemented");
        };

        if let Some(barrier) = barrier {
            self.pipeline_barriers.push(barrier);
            self.current_pipeline_barriers += 1;
        }
    }
}
