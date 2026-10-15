use crate::{
    Result,
    render::{DEPTH_FORMAT, GraphicsDevice, Pipeline, Shader},
};
use alexandria::{
    gpu::{
        VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlag, VulkanCullModeFlag,
        VulkanDescriptorSetLayout, VulkanDynamicState, VulkanFormat, VulkanFrontFace,
        VulkanLogicOp, VulkanPipelineColorBlendAttachmentState,
        VulkanPipelineColorBlendStateCreateInfo, VulkanPipelineDepthStencilStateCreateInfo,
        VulkanPipelineDynamicStateCreateInfo, VulkanPipelineInputAssemblyStateCreateInfo,
        VulkanPipelineMultisampleStateCreateInfo, VulkanPipelineRasterizationStateCreateInfo,
        VulkanPipelineShaderStageCreateInfo, VulkanPipelineVertexInputStateCreateInfo,
        VulkanPipelineViewportStateCreateInfo, VulkanPolygonMode, VulkanPrimitiveTopology,
        VulkanPushConstantRange, VulkanSampleCountFlag, VulkanShaderStageFlag,
    },
    math::{Color4f, Linear},
};
use std::{ffi::CStr, sync::Arc};

impl<Kind> Pipeline<Kind> {
    /// Create a new [`Pipeline`] for a post-processing effect
    pub fn new_post_process(
        name: &'static str,
        fullscreen_quad: &Arc<Shader>,
        fragment_shader: &Arc<Shader>,
        push_constant_size: usize,
        depth_stencil_state: Option<&VulkanPipelineDepthStencilStateCreateInfo>,
        output_formats: &[VulkanFormat],
        descriptor_set_layouts: &[&VulkanDescriptorSetLayout],
        device: &GraphicsDevice,
    ) -> Result<Pipeline<Kind>> {
        assert!(
            push_constant_size <= 128,
            "push constant size must be less than or equal to 128 bytes"
        );

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

        Pipeline::new(
            name,
            descriptor_set_layouts,
            &[VulkanPushConstantRange::new(
                VulkanShaderStageFlag::Fragment,
                0,
                push_constant_size as _,
            )],
            output_formats,
            if depth_stencil_state.is_none() {
                VulkanFormat::Undefined
            } else {
                DEPTH_FORMAT
            },
            &shader_stages,
            vec![fullscreen_quad.clone(), fragment_shader.clone()],
            &vertex_input_state,
            &input_assembly_state,
            &viewport_state,
            &rasterization_state,
            &multisample_state,
            depth_stencil_state,
            &color_blend_state,
            &dynamic_state,
            device,
        )
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
