use crate::{
    Result,
    render::{Pipeline, Shader, frame_graph::SolidColorSkyNode},
};
use alexandria::{
    gpu::{
        VulkanCompareOp, VulkanDevice, VulkanFormat, VulkanPipelineDepthStencilStateCreateInfo,
        VulkanStencilOp, compile_shader,
    },
    math::{Color4f, Linear},
};
use std::sync::Arc;

compile_shader! {
    const FRAGMENT_SHADER = "solid-color-sky.slang",
    frag_main
}

impl SolidColorSkyNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render) fn create_objects(
        pipelines: &mut Vec<Pipeline>,
        fullscreen_quad: &Arc<Shader>,
        swapchain_format: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        assert_eq!(pipelines.len(), 0);

        let shader = Shader::new(&FRAGMENT_SHADER, device)?;

        let depth_stencil_state = VulkanPipelineDepthStencilStateCreateInfo::new(
            0,
            true,
            false,
            VulkanCompareOp::LessOrEqual,
            false,
            false,
            VulkanStencilOp::Keep,
            VulkanStencilOp::Keep,
            VulkanStencilOp::Keep,
            VulkanCompareOp::Always,
            0,
            0,
            0,
            VulkanStencilOp::Keep,
            VulkanStencilOp::Keep,
            VulkanStencilOp::Keep,
            VulkanCompareOp::Always,
            0,
            0,
            0,
            0.0,
            1.0,
        );

        let pipeline = Pipeline::new_post_process(
            fullscreen_quad,
            &shader,
            std::mem::size_of::<Color4f<Linear>>(),
            Some(&depth_stencil_state),
            swapchain_format,
            device,
        )?;

        pipelines.push(pipeline);

        Ok(())
    }
}
