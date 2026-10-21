use crate::render::frame_graph::{
    FrameGraphResources, FrameGraphTransientResourceInfo,
    resources::FrameGraphTransientResourceBuffer,
};

impl FrameGraphResources {
    /// Create a new instance of [`FrameGraphResources`]
    pub fn new(
        native_scale_resources: Vec<FrameGraphTransientResourceInfo>,
    ) -> FrameGraphResources {
        FrameGraphResources {
            external_resources: Vec::new(),
            native_scale: FrameGraphTransientResourceBuffer::new(
                "Native Scale Memory",
                native_scale_resources,
            ),
        }
    }
}
