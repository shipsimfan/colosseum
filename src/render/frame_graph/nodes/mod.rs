use crate::render::{
    RenderData, RenderObjects,
    frame_graph::{FrameGraphResourceId, FrameGraphResourceWriteUsage},
};
use alexandria::{gpu::VulkanCommandBuffer, math::Vector2u};
use r#macro::nodes;

mod r#macro;

nodes![
    /// A node that clears the screen to a solid color
    solid_color_sky::SolidColorSky(SolidColorSkyNode),

    /// A node that renders unlit objects using a forward rendering pipeline
    unlit_forward_render::UnlitForwardRender(UnlitForwardRenderNode),
];
