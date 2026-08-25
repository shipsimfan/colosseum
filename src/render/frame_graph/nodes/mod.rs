use crate::{
    Result,
    render::{
        Pipeline, RenderData, RenderObjects, Shader,
        frame_graph::{FrameGraphResourceId, FrameGraphResourceUsage, FrameGraphResources},
    },
};
use alexandria::gpu::{VulkanCommandBuffer, VulkanDevice, VulkanFormat};
use r#macro::nodes;
use std::sync::Arc;

mod r#macro;

nodes![
    /// A node that changes the render scale of the input image and outputs it to a new image
    render_scale::RenderScale(RenderScaleNode),

    /// A node that clears the screen to a solid color
    solid_color_sky::SolidColorSky(SolidColorSkyNode),

    /// A node that renders unlit objects using a forward rendering pipeline
    unlit_forward_render::UnlitForwardRender(UnlitForwardRenderNode),
];
