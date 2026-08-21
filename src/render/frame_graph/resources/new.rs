use crate::render::frame_graph::{
    Arena, FrameGraphExternalResource, FrameGraphResources, FrameGraphTransientResource,
};
use alexandria::gpu::VulkanDeviceMemory;

impl<'a> FrameGraphResources<'a> {
    /// Create a new [`FrameGraphResources`] manager
    pub fn new(
        mut external: Arena<'a, FrameGraphExternalResource<'a>>,
        transient_render_scale: &'a mut Vec<FrameGraphTransientResource>,
        transient_render_scale_memory: &'a mut Option<VulkanDeviceMemory>,
    ) -> FrameGraphResources<'a> {
        for resource in external.as_mut_slice() {
            resource.reset();
        }

        for resource in transient_render_scale.iter_mut() {
            resource.reset();
        }

        FrameGraphResources {
            external,
            transient_render_scale,
            transient_render_scale_memory,
        }
    }
}
