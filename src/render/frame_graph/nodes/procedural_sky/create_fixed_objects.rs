use crate::{
    Result,
    render::{
        FixedRenderObjects, HDR_FORMAT, Pipeline, Shader, Vertex, frame_graph::ProceduralSkyNode,
    },
};
use alexandria::{
    gpu::{
        VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlag, VulkanCullModeFlag,
        VulkanDescriptorSetLayoutBinding, VulkanDescriptorType, VulkanDevice, VulkanDynamicState,
        VulkanFormat, VulkanFrontFace, VulkanLogicOp, VulkanPipelineColorBlendAttachmentState,
        VulkanPipelineColorBlendStateCreateInfo, VulkanPipelineDynamicStateCreateInfo,
        VulkanPipelineInputAssemblyStateCreateInfo, VulkanPipelineMultisampleStateCreateInfo,
        VulkanPipelineRasterizationStateCreateInfo, VulkanPipelineShaderStageCreateInfo,
        VulkanPipelineVertexInputStateCreateInfo, VulkanPipelineViewportStateCreateInfo,
        VulkanPolygonMode, VulkanPrimitiveTopology, VulkanSampleCountFlag, VulkanShaderStageFlag,
        compile_shader,
    },
    math::{Color4f, Linear},
};

compile_shader! {
    const SHADER = "procedural-sky.slang",
    vert_main,
    frag_main
}

impl ProceduralSkyNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render::frame_graph::nodes) fn create_fixed_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        _: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Create the descriptor set layout for the camera
        fixed_render_objects.add_descriptor_set_layout(
            &[VulkanDescriptorSetLayoutBinding::new(
                0,
                VulkanDescriptorType::UniformBuffer,
                1,
                VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
            )],
            1,
            FixedRenderObjects::CAMERA_DESCRIPTOR_SET_LAYOUT,
            device,
        )?;

        // Create the pipeline
        let shader = Shader::new(&SHADER, device)?;

        let pipeline = Pipeline::new(
            &[fixed_render_objects
                .descriptor_set_layout(FixedRenderObjects::CAMERA_DESCRIPTOR_SET_LAYOUT)],
            &[],
            &[HDR_FORMAT],
            VulkanFormat::Undefined,
            &[
                VulkanPipelineShaderStageCreateInfo::new(
                    0,
                    VulkanShaderStageFlag::Vertex,
                    shader.module(),
                    c"vert_main",
                    None,
                ),
                VulkanPipelineShaderStageCreateInfo::new(
                    0,
                    VulkanShaderStageFlag::Fragment,
                    shader.module(),
                    c"frag_main",
                    None,
                ),
            ],
            vec![shader.clone()],
            &VulkanPipelineVertexInputStateCreateInfo::new(
                &Vertex::POSITION_ONLY_ATTRIBUTE_DESCRIPTORS,
                &Vertex::BINDING_DESCRIPTORS,
            ),
            &VulkanPipelineInputAssemblyStateCreateInfo::new(
                VulkanPrimitiveTopology::TriangleList,
                false,
            ),
            &VulkanPipelineViewportStateCreateInfo::new_dynamic(1, 1),
            &VulkanPipelineRasterizationStateCreateInfo::new(
                false,
                false,
                VulkanPolygonMode::Fill,
                VulkanCullModeFlag::Front,
                VulkanFrontFace::CounterClockwise,
                false,
                0.0,
                0.0,
                0.0,
                1.0,
            ),
            &VulkanPipelineMultisampleStateCreateInfo::new(
                VulkanSampleCountFlag::_1,
                false,
                0.0,
                None,
                false,
                false,
            ),
            None,
            &VulkanPipelineColorBlendStateCreateInfo::new(
                0,
                false,
                VulkanLogicOp::Copy,
                &[VulkanPipelineColorBlendAttachmentState::new(
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
                )],
                Color4f::<Linear>::CLEAR,
            ),
            &VulkanPipelineDynamicStateCreateInfo::new(&[
                VulkanDynamicState::Viewport,
                VulkanDynamicState::Scissor,
            ]),
            device,
        )?;

        fixed_render_objects.add_pipeline(pipeline, FixedRenderObjects::PROCEDURAL_SKY_PIPELINE);
        Ok(())
    }
}
