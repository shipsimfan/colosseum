use crate::render::frame_graph::{
    Arena, FrameGraphDynamicTransientResourceInfo, FrameGraphExternalResource,
    FrameGraphResourceBuilder,
};

impl<'a> FrameGraphResourceBuilder<'a> {
    /// Finish building and return the finalized resources
    pub(in crate::render::frame_graph) fn finish(
        self,
    ) -> (
        Arena<'a, FrameGraphExternalResource<'a>>,
        &'a mut Vec<FrameGraphDynamicTransientResourceInfo>,
        &'a mut Vec<FrameGraphDynamicTransientResourceInfo>,
    ) {
        (
            self.external,
            self.transient_render_scale,
            self.transient_native_scale,
        )
    }
}
