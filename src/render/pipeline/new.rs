use crate::{
    Error, Result,
    render::{Pipeline, Shader},
};
use alexandria::{
    gpu::{
        VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlag, VulkanCullModeFlag,
        VulkanDevice, VulkanDynamicState, VulkanFormat, VulkanFrontFace, VulkanLogicOp,
        VulkanPipelineColorBlendAttachmentState, VulkanPipelineColorBlendStateCreateInfo,
        VulkanPipelineDepthStencilStateCreateInfo, VulkanPipelineDynamicStateCreateInfo,
        VulkanPipelineInputAssemblyStateCreateInfo, VulkanPipelineMultisampleStateCreateInfo,
        VulkanPipelineRasterizationStateCreateInfo, VulkanPipelineRenderingCreateInfo,
        VulkanPipelineShaderStageCreateInfo, VulkanPipelineVertexInputStateCreateInfo,
        VulkanPipelineViewportStateCreateInfo, VulkanPolygonMode, VulkanPrimitiveTopology,
        VulkanPushConstantRange, VulkanSampleCountFlag, VulkanShaderStageFlag,
    },
    math::{Color4f, Linear},
};
use std::{ffi::CStr, sync::Arc};

impl Pipeline {
    /// Create a new [`Pipeline`] for a post-processing effect
    pub fn new_post_process(
        fullscreen_quad: &Arc<Shader>,
        fragment_shader: &Arc<Shader>,
        push_constant_size: usize,
        depth_stencil_state: Option<&VulkanPipelineDepthStencilStateCreateInfo>,
        swapchain_format: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<Pipeline> {
        assert!(
            push_constant_size <= 128,
            "push constant size must be less than or equal to 128 bytes"
        );

        // Craete the pipeline layout
        let push_constant_range = [VulkanPushConstantRange::new(
            VulkanShaderStageFlag::Fragment,
            0,
            push_constant_size as _,
        )];

        let pipeline_layout = device
            .create_pipeline_layout(0, &[], &push_constant_range)
            .map_err(Error::new_inner)?;

        // Define the shader stages
        let shader_stages = [
            create_shader_stage(fullscreen_quad, VulkanShaderStageFlag::Vertex, c"vert_main"),
            create_shader_stage(
                fragment_shader,
                VulkanShaderStageFlag::Fragment,
                c"frag_main",
            ),
        ];

        // Create the dynamic state (viewport)
        let dynamic_state = VulkanPipelineDynamicStateCreateInfo::new(&[
            VulkanDynamicState::Viewport,
            VulkanDynamicState::Scissor,
        ]);
        let viewport_state = VulkanPipelineViewportStateCreateInfo::new_dynamic(1, 1);

        // Setup the vertex input and input assembly states
        let vertex_input_state = VulkanPipelineVertexInputStateCreateInfo::new(&[], &[]);
        let input_assembly_state = VulkanPipelineInputAssemblyStateCreateInfo::new(
            VulkanPrimitiveTopology::TriangleFan,
            false,
        );

        // Create the rasterization state
        let rasterization_state = VulkanPipelineRasterizationStateCreateInfo::new(
            false,
            false,
            VulkanPolygonMode::Fill,
            VulkanCullModeFlag::Back,
            VulkanFrontFace::CounterClockwise,
            false,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        // Create the multisample state
        let multisample_state = VulkanPipelineMultisampleStateCreateInfo::new(
            VulkanSampleCountFlag::_1,
            false,
            0.0,
            None,
            false,
            false,
        );

        // Create the color blend state
        let color_blend_attachment = [VulkanPipelineColorBlendAttachmentState::new(
            false,
            VulkanBlendFactor::Zero,
            VulkanBlendFactor::Zero,
            VulkanBlendOp::Add,
            VulkanBlendFactor::Zero,
            VulkanBlendFactor::Zero,
            VulkanBlendOp::Add,
            VulkanColorComponentFlag::R
                | VulkanColorComponentFlag::G
                | VulkanColorComponentFlag::B
                | VulkanColorComponentFlag::A,
        )];
        let color_blend_state = VulkanPipelineColorBlendStateCreateInfo::new(
            0,
            false,
            VulkanLogicOp::Copy,
            &color_blend_attachment,
            Color4f::<Linear>::CLEAR,
        );

        // Actually create the graphics pipeline
        let pipeline = device
            .create_graphics_pipeline(
                [&mut VulkanPipelineRenderingCreateInfo::new(
                    0,
                    &[swapchain_format],
                    if depth_stencil_state.is_none() {
                        VulkanFormat::Undefined
                    } else {
                        VulkanFormat::D32SFloat
                    },
                    VulkanFormat::Undefined,
                ) as _],
                None,
                0,
                &shader_stages,
                Some(&vertex_input_state),
                Some(&input_assembly_state),
                None,
                Some(&viewport_state),
                Some(&rasterization_state),
                Some(&multisample_state),
                depth_stencil_state,
                Some(&color_blend_state),
                Some(&dynamic_state),
                Some(&pipeline_layout),
                None,
                0,
                None,
                0,
            )
            .map_err(Error::new_inner)?;

        Ok(Pipeline {
            pipeline,
            layout: pipeline_layout,
            shader: vec![fullscreen_quad.clone(), fragment_shader.clone()],
        })
    }
}

/// Create a single shader stage for the graphics pipeline
fn create_shader_stage<'a>(
    shader: &'a Arc<Shader>,
    stage: VulkanShaderStageFlag,
    name: &'a CStr,
) -> VulkanPipelineShaderStageCreateInfo<'a> {
    VulkanPipelineShaderStageCreateInfo::new(0, stage, shader.module(), name, None)
}
