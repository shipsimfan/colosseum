use crate::render::{
    FrameGraphTransientBuffer,
    frame_graph::{
        Arena, FrameGraphExternalResource, FrameGraphResources, resources::FrameGraphResourceList,
    },
};

impl<'a> FrameGraphResources<'a> {
    /// Create a new [`FrameGraphResources`] manager
    pub fn new(
        mut external: Arena<'a, FrameGraphExternalResource<'a>>,
        transient_buffer: &'a mut FrameGraphTransientBuffer,
    ) -> FrameGraphResources<'a> {
        for resource in external.as_mut_slice() {
            resource.reset();
        }

        for resource in transient_buffer.transient_native_scale.iter_mut() {
            resource.reset();
        }

        for resource in transient_buffer.transient_render_scale.iter_mut() {
            resource.reset();
        }

        FrameGraphResources {
            external,
            epoch: &mut transient_buffer.epoch,
            render_scale_transients: FrameGraphResourceList::new(
                &mut transient_buffer.transient_render_scale,
                &mut transient_buffer.transient_render_scale_memory,
            ),
            native_scale_transients: FrameGraphResourceList::new(
                &mut transient_buffer.transient_native_scale,
                &mut transient_buffer.transient_native_scale_memory,
            ),
        }
    }
}
