use crate::{
    Result,
    render::{FixedRenderObjects, HDR_FORMAT, Pipeline, Shader, frame_graph::SolidColorSkyNode},
};
use alexandria::{
    gpu::{VulkanDevice, VulkanFormat, compile_shader},
    math::{Color4f, Linear},
};
