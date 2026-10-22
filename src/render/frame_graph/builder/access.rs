use crate::render::frame_graph::{
    FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceUsage,
};

impl<'a> FrameGraphBuilder<'a> {
    /// Specify a requested access for a resource within the frame graph
    pub fn access(&mut self, id: FrameGraphResourceId, usage: FrameGraphResourceUsage) {
        let mut resource = self.resources.get(id);

        // Update resources for the render pass for this node
        match usage {
            FrameGraphResourceUsage::ColorAttachment => {
                if self.color_attachment.is_some() {
                    panic!("multiple color attachments defined for a node");
                }

                let load_op = resource.set_color();
                self.color_attachment = Some((id, load_op));
            }
            _ => {}
        }

        // Transition the resource and a pipeline barrier if necessary
        if let Some(barrier) = resource.transition(id, usage.state()) {
            self.pipeline_barriers.push(barrier);
            self.current_pipeline_barriers += 1;
        }
    }
}
