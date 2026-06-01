use crate::render::{FrameContext, RenderData, frame_graph::UnlitForwardPassNode};

impl UnlitForwardPassNode {
    /// Execute the unlit forward pass, rendering all unlit objects in the scene
    pub(in crate::render::frame_graph) fn execute(
        &self,
        frame: &mut FrameContext,
        render_data: &RenderData,
        resources: &FrameGraphResources,
    ) {
        let clear_color = render_data.clear_color().with_alpha(1.0);
        let resource = &resources[self.output];

        frame.cmd_begin_rendering(resource.handle(), resource.size(), clear_color);

        frame.cmd_end_rendering();
    }
}
