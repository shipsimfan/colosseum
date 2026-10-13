use crate::{
    Result,
    render::{
        FixedRenderObjects, Pipeline, Shader, ShadowMapBuffer, Vertex,
        frame_graph::{ShadowMapLight, ShadowMapNode, nodes::shadow_map::PushConstants},
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
use std::sync::Arc;

compile_shader! {
    const SHADER = "shadow-map.slang",
    vert_main
}

compile_shader! {
    const POINT_LIGHT_SHADER = "point-light-shadow-map.slang",
    vert_main,
    frag_main
}

impl<L: ShadowMapLight> ShadowMapNode<L> {
    /// Create the persistent objects that are used by this node
    pub(in crate::render::frame_graph::nodes) fn create_fixed_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        _: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        if L::PRIMARY {
            create_shadow_map_descriptor_set_layout(fixed_render_objects, device)?;
            create_point_light_shadow_map_descriptor_set_layout(fixed_render_objects, device)?;

            let common_shader = Shader::new(&SHADER, device)?;
            create_shadow_map_pipeline(
                fixed_render_objects,
                "Shadow Map Pipeline",
                FixedRenderObjects::SHADOW_MAP_DESCRIPTOR_SET_LAYOUT,
                &[VulkanPipelineShaderStageCreateInfo::new(
                    0,
                    VulkanShaderStageFlag::Vertex,
                    common_shader.module(),
                    c"vert_main",
                    None,
                )],
                vec![common_shader.clone()],
                FixedRenderObjects::SHADOW_MAP_PIPELINE,
                device,
            )?;

            let point_light_shader = Shader::new(&POINT_LIGHT_SHADER, device)?;
            create_shadow_map_pipeline(
                fixed_render_objects,
                "Point Light Shadow Map Pipeline",
                FixedRenderObjects::POINT_LIGHT_SHADOW_MAP_DESCRIPTOR_SET_LAYOUT,
                &[
                    VulkanPipelineShaderStageCreateInfo::new(
                        0,
                        VulkanShaderStageFlag::Vertex,
                        point_light_shader.module(),
                        c"vert_main",
                        None,
                    ),
                    VulkanPipelineShaderStageCreateInfo::new(
                        0,
                        VulkanShaderStageFlag::Fragment,
                        point_light_shader.module(),
                        c"frag_main",
                        None,
                    ),
                ],
                vec![point_light_shader.clone()],
                FixedRenderObjects::POINT_LIGHT_SHADOW_MAP_PIPELINE,
                device,
            )?;
        }

        Ok(())
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

fn create_point_light_shadow_map_descriptor_set_layout(
    fixed_render_objects: &mut FixedRenderObjects,
    device: &VulkanDevice,
) -> Result<()> {
    fixed_render_objects.add_descriptor_set_layout(
        c"Point Light Shadow Map Descriptor Set Layout",
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
            VulkanDescriptorSetLayoutBinding::new(
                2,
                VulkanDescriptorType::StorageBuffer,
                1,
                VulkanShaderStageFlag::Fragment,
            ),
        ],
        2,
        FixedRenderObjects::POINT_LIGHT_SHADOW_MAP_DESCRIPTOR_SET_LAYOUT,
        device,
    )
}

fn create_shadow_map_pipeline(
    fixed_render_objects: &mut FixedRenderObjects,
    name: &'static str,
    descriptor_set_layout: usize,
    shader_stages: &[VulkanPipelineShaderStageCreateInfo],
    shaders: Vec<Arc<Shader>>,
    index: usize,
    device: &VulkanDevice,
) -> Result<()> {
    let pipeline = Pipeline::new(
        name,
        &[fixed_render_objects.descriptor_set_layout(descriptor_set_layout)],
        &[VulkanPushConstantRange::new(
            VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
            0,
            std::mem::size_of::<PushConstants>() as _,
        )],
        &[],
        ShadowMapBuffer::FORMAT,
        shader_stages,
        shaders,
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
    fixed_render_objects.add_pipeline(pipeline, index);
    Ok(())
}
