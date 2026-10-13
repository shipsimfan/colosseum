use crate::render::{
    FrameGraphTransientBuffer,
    frame_graph::{Arena, FrameGraphExternalResource, FrameGraphResources},
};

impl<'a> FrameGraphResources<'a> {
    /// Create a new [`FrameGraphResources`] manager
    pub fn new(
        mut external: Arena<'a, FrameGraphExternalResource<'a>>,
        transient: &'a mut FrameGraphTransientBuffer,
    ) -> FrameGraphResources<'a> {
        for resource in external.as_mut_slice() {
            resource.reset();
        }

        for resource in &mut transient.native_scale {
            resource.reset();
        }

        for resource in &mut transient.render_scale {
            resource.reset();
        }

        FrameGraphResources {
            external,
            transient,
        }
    }
}
