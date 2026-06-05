use crate::render::{
    RenderData,
    frame_graph::{FrameGraphResourceId, FrameGraphResourceWriteUsage},
};
use alexandria::gpu::VulkanCommandBuffer;
use r#macro::nodes;

mod r#macro;

nodes![solid_color_sky::SolidColorSky(SolidColorSkyNode),];
