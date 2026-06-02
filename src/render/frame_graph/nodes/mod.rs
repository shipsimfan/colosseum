use crate::render::{
    RenderData,
    frame_graph::{FrameGraphResourceId, FrameGraphResources},
};
use alexandria::gpu::VulkanCommandBuffer;
use r#macro::nodes;

mod r#macro;

nodes![unlit_forward_pass::UnlitForwardPass(UnlitForwardPassNode),];
