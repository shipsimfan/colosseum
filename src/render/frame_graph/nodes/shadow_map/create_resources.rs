use crate::{
    Result,
    render::{
        DEPTH_FORMAT, DescriptorSetLayouts, GraphicsDevice, Pipeline, ShadowMapLight, Vertex,
        frame_graph::{ShadowMapNode, nodes::shadow_map::PushConstants},
    },
};
use alexandria::{
    gpu::{
        VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlag, VulkanCompareOp,
        VulkanCullModeFlag, VulkanDynamicState, VulkanFrontFace, VulkanLogicOp,
        VulkanPipelineColorBlendAttachmentState, VulkanPipelineColorBlendStateCreateInfo,
        VulkanPipelineDepthStencilStateCreateInfo, VulkanPipelineDynamicStateCreateInfo,
        VulkanPipelineInputAssemblyStateCreateInfo, VulkanPipelineMultisampleStateCreateInfo,
        VulkanPipelineRasterizationStateCreateInfo, VulkanPipelineVertexInputStateCreateInfo,
        VulkanPipelineViewportStateCreateInfo, VulkanPolygonMode, VulkanPrimitiveTopology,
        VulkanPushConstantRange, VulkanSampleCountFlag, VulkanShaderStageFlag, VulkanStencilOp,
    },
    math::{Color4f, Linear},
};
use colosseum_ecs::World;

impl<L: ShadowMapLight> ShadowMapNode<L> {
    /// Create the resources required by the [`ShadowMapNode`]
    pub fn create_resources(world: &mut World, device: &GraphicsDevice) -> Result<()> {
        let shader = L::create_shader(device)?;

        let descriptor_set_layous = world.get_resource::<DescriptorSetLayouts>();

        world.insert_permanent_resource(Pipeline::<L>::new(
            &format!("{} Pipeline", L::NAME),
            &[descriptor_set_layous.get::<L::DescriptorSetLayout>()],
            &[VulkanPushConstantRange::new(
                VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
                0,
                std::mem::size_of::<PushConstants>() as _,
            )],
            &[],
            DEPTH_FORMAT,
            &L::shader_stages(&shader),
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
        )?);
        Ok(())
    }
}
