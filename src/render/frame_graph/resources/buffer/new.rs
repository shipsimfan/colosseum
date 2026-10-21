use crate::render::{
    MAX_FRAMES_IN_FLIGHT,
    frame_graph::{FrameGraphTransientResourceInfo, resources::FrameGraphTransientResourceBuffer},
};
use alexandria::math::Vector2u;

impl FrameGraphTransientResourceBuffer {
    /// Create a new [`FrameGraphTransientResourceBuffer`]
    pub fn new(
        memory_name: &'static str,
        info: Vec<FrameGraphTransientResourceInfo>,
    ) -> FrameGraphTransientResourceBuffer {
        let image_views = [
            Vec::with_capacity(info.len()),
            Vec::with_capacity(info.len()),
            Vec::with_capacity(info.len()),
        ];

        FrameGraphTransientResourceBuffer {
            memory_name,
            info,
            size: [Vector2u::ZERO; MAX_FRAMES_IN_FLIGHT],
            image_views,
            memory: [None, None, None],
        }
    }
}
