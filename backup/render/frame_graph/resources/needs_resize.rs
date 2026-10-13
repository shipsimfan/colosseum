use crate::render::frame_graph::FrameGraphResources;

impl<'a> FrameGraphResources<'a> {
    /// Does this set of transients need a resize?
    pub fn needs_resize(&self, epoch: u64) -> bool {
        self.transient.epoch != epoch
    }
}
