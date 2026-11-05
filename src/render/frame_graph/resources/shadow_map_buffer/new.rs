use crate::render::{
    MAX_FRAMES_IN_FLIGHT,
    frame_graph::{FrameGraphTransientResourceInfo, resources::FrameGraphShadowMapBuffer},
};
use alexandria::math::Vector2u;

impl FrameGraphShadowMapBuffer {
    /// Create a new [`FrameGraphShadowMapBuffer`]
    pub fn new(
        memory_name: &'static str,
        info: FrameGraphTransientResourceInfo,
    ) -> FrameGraphShadowMapBuffer {
        FrameGraphShadowMapBuffer {
            memory_name,
            info,
            size: [Vector2u::ZERO; MAX_FRAMES_IN_FLIGHT],
            array_layers: [0; MAX_FRAMES_IN_FLIGHT],
            image_views: [const { Vec::new() }; MAX_FRAMES_IN_FLIGHT],
            full_image_view: [const { None }; MAX_FRAMES_IN_FLIGHT],
            memory: [const { None }; MAX_FRAMES_IN_FLIGHT],
        }
    }
}
