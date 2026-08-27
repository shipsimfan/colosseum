use crate::render::FrameGraphTransientBuffer;

impl FrameGraphTransientBuffer {
    /// Create a new [`FrameGraphTransientBuffer`]
    pub fn new() -> FrameGraphTransientBuffer {
        FrameGraphTransientBuffer {
            epoch: 0,
            transient_render_scale: Vec::new(),
            transient_render_scale_memory: None,
            transient_native_scale: Vec::new(),
            transient_native_scale_memory: None,
        }
    }
}
