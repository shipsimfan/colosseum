use crate::render::frame_graph::{FrameGraphResource, FrameGraphResources};

impl<'a> FrameGraphResources<'a> {
    /// Create a new [`FrameGraphResources`] with the given swapchain image resource
    pub(in crate::render::frame_graph::resources) fn new(
        transient: &'a mut Vec<FrameGraphResource<'a>>,
        external: &'a mut Vec<FrameGraphResource<'a>>,
    ) -> FrameGraphResources<'a> {
        FrameGraphResources {
            transient,
            external,
        }
    }
}
