use crate::{
    Error, Result,
    render::{RenderMaterial, Shader, Vertex},
};
use alexandria::{
    gpu::{
        VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlag, VulkanCullModeFlag,
        VulkanDevice, VulkanDynamicState, VulkanFormat, VulkanFrontFace, VulkanLogicOp,
        VulkanPipelineColorBlendAttachmentState, VulkanPipelineColorBlendStateCreateInfo,
        VulkanPipelineDynamicStateCreateInfo, VulkanPipelineInputAssemblyStateCreateInfo,
        VulkanPipelineLayout, VulkanPipelineMultisampleStateCreateInfo,
        VulkanPipelineRasterizationStateCreateInfo, VulkanPipelineRenderingCreateInfo,
        VulkanPipelineShaderStageCreateInfo, VulkanPipelineVertexInputStateCreateInfo,
        VulkanPipelineViewportStateCreateInfo, VulkanPolygonMode, VulkanPrimitiveTopology,
        VulkanSampleCountFlag, VulkanShaderStageFlag,
    },
    math::{Color4f, Linear},
};
use std::{ffi::CStr, sync::Arc};

const VERTEX_ENTRY: &CStr = c"vert_main";
const FRAGMENT_ENTRY: &CStr = c"frag_main";

impl RenderMaterial {
    /// Create a new [`RenderMaterial`]
    pub(in crate::render::material) fn new(
        shader: &Arc<Shader>,
        pipeline_layout: &VulkanPipelineLayout,
        swapchain_format: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<RenderMaterial> {
        // Define the shader stages
        let shader_stages = [
            create_shader_stage(shader, VulkanShaderStageFlag::Vertex, VERTEX_ENTRY),
            create_shader_stage(shader, VulkanShaderStageFlag::Fragment, FRAGMENT_ENTRY),
        ];

        // Create the dynamic state (viewport)
        let dynamic_state = VulkanPipelineDynamicStateCreateInfo::new(&[
            VulkanDynamicState::Viewport,
            VulkanDynamicState::Scissor,
        ]);
        let viewport_state = VulkanPipelineViewportStateCreateInfo::new_dynamic(1, 1);

        // Setup the vertex input and input assembly states
        let vertex_input_state = VulkanPipelineVertexInputStateCreateInfo::new(
            &Vertex::ATTRIBUTE_DESCRIPTORS,
            &Vertex::BINDING_DESCRIPTORS,
        );
        let input_assembly_state = VulkanPipelineInputAssemblyStateCreateInfo::new(
            VulkanPrimitiveTopology::TriangleList,
            false,
        );

        // Create the rasterization state
        let rasterization_state = VulkanPipelineRasterizationStateCreateInfo::new(
            false,
            false,
            VulkanPolygonMode::Fill,
            VulkanCullModeFlag::Back,
            VulkanFrontFace::Clockwise,
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
                    VulkanFormat::Undefined,
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
                None,
                Some(&color_blend_state),
                Some(&dynamic_state),
                Some(pipeline_layout),
                None,
                0,
                None,
                0,
            )
            .map_err(Error::new_inner)?;

        Ok(RenderMaterial {
            pipeline,
            shader: shader.clone(),
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
