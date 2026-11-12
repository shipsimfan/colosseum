use crate::{
    Result,
    render::{
        CameraDsl, DescriptorSetLayouts, GraphicsDevice, HDR_FORMAT, Pipeline, Shader, Vertex,
        frame_graph::{ProceduralSkyNode, nodes::procedural_sky::PushConstants},
    },
};
use alexandria::{
    gpu::{
        VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlag, VulkanCullModeFlag,
        VulkanDynamicState, VulkanFormat, VulkanFrontFace, VulkanLogicOp,
        VulkanPipelineColorBlendAttachmentState, VulkanPipelineColorBlendStateCreateInfo,
        VulkanPipelineDynamicStateCreateInfo, VulkanPipelineInputAssemblyStateCreateInfo,
        VulkanPipelineMultisampleStateCreateInfo, VulkanPipelineRasterizationStateCreateInfo,
        VulkanPipelineShaderStageCreateInfo, VulkanPipelineVertexInputStateCreateInfo,
        VulkanPipelineViewportStateCreateInfo, VulkanPolygonMode, VulkanPrimitiveTopology,
        VulkanPushConstantRange, VulkanSampleCountFlag, VulkanShaderStageFlag, compile_shader,
    },
    math::{Color4f, Linear},
};
use colosseum_ecs::World;

compile_shader! {
    const SHADER = "procedural-sky.slang",
    vert_main,
    frag_main
}

impl ProceduralSkyNode {
    /// Create the resources required by the [`ProceduralSkyNode`]
    pub fn create_resources(world: &mut World, device: &GraphicsDevice) -> Result<()> {
        let shader = Shader::new(&SHADER, device)?;

        let descriptor_set_layouts = world.get_resource::<DescriptorSetLayouts>();
        let descriptor_set_layout = descriptor_set_layouts.get::<CameraDsl>();

        let pipeline = Pipeline::<ProceduralSkyNode>::new(
            "Procedural Sky Pipeline",
            &[descriptor_set_layout],
            &[VulkanPushConstantRange::new(
                VulkanShaderStageFlag::Fragment,
                0,
                std::mem::size_of::<PushConstants>() as _,
            )],
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
                VulkanFrontFace::Clockwise,
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

        world.insert_permanent_resource(pipeline);

        Ok(())
    }
}
