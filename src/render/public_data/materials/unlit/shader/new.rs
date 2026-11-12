use crate::{
    Result,
    render::{DEPTH_FORMAT, GraphicsDevice, HDR_FORMAT, Shader, ShaderCode, UnlitShader, Vertex},
};
use alexandria::{
    gpu::{
        VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlag, VulkanCompareOp,
        VulkanCullModeFlag, VulkanDynamicState, VulkanFormat, VulkanFrontFace, VulkanLogicOp,
        VulkanPipelineColorBlendAttachmentState, VulkanPipelineColorBlendStateCreateInfo,
        VulkanPipelineDepthStencilStateCreateInfo, VulkanPipelineDynamicStateCreateInfo,
        VulkanPipelineInputAssemblyStateCreateInfo, VulkanPipelineLayout,
        VulkanPipelineMultisampleStateCreateInfo, VulkanPipelineRasterizationStateCreateInfo,
        VulkanPipelineRenderingCreateInfo, VulkanPipelineShaderStageCreateInfo,
        VulkanPipelineVertexInputStateCreateInfo, VulkanPipelineViewportStateCreateInfo,
        VulkanPolygonMode, VulkanPrimitiveTopology, VulkanSampleCountFlag, VulkanShaderStageFlag,
        VulkanStencilOp,
    },
    math::{Color4f, Linear},
};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;
use std::{ffi::CStr, sync::Arc};

impl UnlitShader {
    pub(in crate::render::public_data::materials::unlit) fn new<const N: usize>(
        code: &ShaderCode<N>,
        pipeline_layout: &VulkanPipelineLayout,
        device: &GraphicsDevice,
    ) -> Result<Arc<UnlitShader>> {
        let shader = Shader::new(code, device)?;

        // Define the shader stages
        let shader_stages = [
            create_shader_stage(
                &shader,
                VulkanShaderStageFlag::Vertex,
                code.entry_points()[0],
            ),
            create_shader_stage(
                &shader,
                VulkanShaderStageFlag::Fragment,
                code.entry_points()[1],
            ),
        ];

        // Create the dynamic state (viewport)
        let dynamic_state = VulkanPipelineDynamicStateCreateInfo::new(&[
            VulkanDynamicState::Viewport,
            VulkanDynamicState::Scissor,
        ]);
        let viewport_state = VulkanPipelineViewportStateCreateInfo::new_dynamic(1, 1);

        // Setup the vertex input and input assembly states
        let vertex_input_state = VulkanPipelineVertexInputStateCreateInfo::new(
            &Vertex::UNLIT_ATTRIBUTE_DESCRIPTORS,
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

        // Create the depth stencil state
        let depth_stencil_state = VulkanPipelineDepthStencilStateCreateInfo::new(
            0,
            true,
            true,
            VulkanCompareOp::Greater,
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
        let mut pipeline = device
            .create_graphics_pipeline(
                [&mut VulkanPipelineRenderingCreateInfo::new(
                    0,
                    &[HDR_FORMAT],
                    DEPTH_FORMAT,
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
                Some(&depth_stencil_state),
                Some(&color_blend_state),
                Some(&dynamic_state),
                Some(pipeline_layout),
                None,
                0,
                None,
                0,
            )
            .map_err(|error| {
                new_error!(
                    "unable to create an unlit pipeline for \"{}\" - {}",
                    code.file_name(),
                    error
                )
            })?;

        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut pipeline,
                &CString::new(format!("\"{}\" Unlit Pipeline", code.file_name())).unwrap(),
            )
            .map_err(|error| {
                new_error!(
                    "unable to name pipeline for \"{}\" - {}",
                    code.file_name(),
                    error
                )
            })?;

        Ok(Arc::new(UnlitShader { pipeline, shader }))
    }
}

/// Create a single shader stage for the graphics pipeline
fn create_shader_stage<'a>(
    shader: &'a Shader,
    stage: VulkanShaderStageFlag,
    name: &'a CStr,
) -> VulkanPipelineShaderStageCreateInfo<'a> {
    VulkanPipelineShaderStageCreateInfo::new(0, stage, shader.module(), name, None)
}
