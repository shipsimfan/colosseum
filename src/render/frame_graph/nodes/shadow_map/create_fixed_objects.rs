use crate::{
    Result,
    render::{
        FixedRenderObjects, Pipeline, Shader, ShadowMapBuffer, Vertex,
        frame_graph::{ShadowMapNode, nodes::shadow_map::PushConstants},
    },
};
use alexandria::{
    gpu::{
        VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlag, VulkanCompareOp,
        VulkanCullModeFlag, VulkanDescriptorSetLayoutBinding, VulkanDescriptorType, VulkanDevice,
        VulkanDynamicState, VulkanFormat, VulkanFrontFace, VulkanLogicOp,
        VulkanPipelineColorBlendAttachmentState, VulkanPipelineColorBlendStateCreateInfo,
        VulkanPipelineDepthStencilStateCreateInfo, VulkanPipelineDynamicStateCreateInfo,
        VulkanPipelineInputAssemblyStateCreateInfo, VulkanPipelineMultisampleStateCreateInfo,
        VulkanPipelineRasterizationStateCreateInfo, VulkanPipelineShaderStageCreateInfo,
        VulkanPipelineVertexInputStateCreateInfo, VulkanPipelineViewportStateCreateInfo,
        VulkanPolygonMode, VulkanPrimitiveTopology, VulkanPushConstantRange, VulkanSampleCountFlag,
        VulkanShaderStageFlag, VulkanStencilOp, compile_shader,
    },
    math::{Color4f, Linear},
};

compile_shader! {
    const SHADER = "shadow-map.slang",
    vert_main
}

impl ShadowMapNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render::frame_graph::nodes) fn create_fixed_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        _: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        create_shadow_map_descriptor_set_layout(fixed_render_objects, device)?;
        create_shadow_map_pipeline(fixed_render_objects, device)
    }
}

fn create_shadow_map_descriptor_set_layout(
    fixed_render_objects: &mut FixedRenderObjects,
    device: &VulkanDevice,
) -> Result<()> {
    fixed_render_objects.add_descriptor_set_layout(
        c"Shadow Map Descriptor Set Layout",
        &[
            VulkanDescriptorSetLayoutBinding::new(
                0,
                VulkanDescriptorType::StorageBuffer,
                1,
                VulkanShaderStageFlag::Vertex,
            ),
            VulkanDescriptorSetLayoutBinding::new(
                1,
                VulkanDescriptorType::StorageBuffer,
                1,
                VulkanShaderStageFlag::Vertex,
            ),
        ],
        2,
        FixedRenderObjects::SHADOW_MAP_DESCRIPTOR_SET_LAYOUT,
        device,
    )
}

fn create_shadow_map_pipeline(
    fixed_render_objects: &mut FixedRenderObjects,
    device: &VulkanDevice,
) -> Result<()> {
    let shader = Shader::new(&SHADER, device)?;

    let pipeline = Pipeline::new(
        "Shadow Map Pipeline",
        &[fixed_render_objects
            .descriptor_set_layout(FixedRenderObjects::SHADOW_MAP_DESCRIPTOR_SET_LAYOUT)],
        &[VulkanPushConstantRange::new(
            VulkanShaderStageFlag::Vertex,
            0,
            std::mem::size_of::<PushConstants>() as _,
        )],
        &[],
        ShadowMapBuffer::FORMAT,
        &[VulkanPipelineShaderStageCreateInfo::new(
            0,
            VulkanShaderStageFlag::Vertex,
            shader.module(),
            c"vert_main",
            None,
        )],
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
        Some(&VulkanPipelineDepthStencilStateCreateInfo::new(
            0,
            true,
            true,
            VulkanCompareOp::Less,
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
        )),
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
    fixed_render_objects.add_pipeline(pipeline, FixedRenderObjects::SHADOW_MAP_PIPELINE);
    Ok(())
}
