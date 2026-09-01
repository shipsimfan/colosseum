use crate::{
    Result,
    render::{FixedRenderObjects, HDR_FORMAT, Pipeline, Shader, frame_graph::SolidColorSkyNode},
};
use alexandria::{
    gpu::{
        VulkanCompareOp, VulkanDevice, VulkanFormat, VulkanPipelineDepthStencilStateCreateInfo,
        VulkanStencilOp, compile_shader,
    },
    math::{Color4f, Linear},
};

compile_shader! {
    const FRAGMENT_SHADER = "solid-color-sky.slang",
    frag_main
}

impl SolidColorSkyNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render::frame_graph::nodes) fn create_fixed_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        _: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Create the pipeline
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
            fixed_render_objects.fullscreen_quad(),
            &shader,
            std::mem::size_of::<Color4f<Linear>>(),
            Some(&depth_stencil_state),
            &[HDR_FORMAT],
            &[],
            device,
        )?;

        fixed_render_objects.add_pipeline(pipeline, FixedRenderObjects::SOLID_COLOR_SKY_PIPELINE);
        Ok(())
    }
}
