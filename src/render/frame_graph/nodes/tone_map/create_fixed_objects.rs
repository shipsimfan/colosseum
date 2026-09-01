use crate::{
    Error, Result,
    render::{
        FixedRenderObjects, Pipeline, SDR_FORMAT, Shader,
        frame_graph::{ToneMapNode, nodes::tone_map::PushConstants},
    },
};
use alexandria::gpu::{
    VulkanBorderColor, VulkanCompareOp, VulkanDescriptorSetLayoutBinding, VulkanDescriptorType,
    VulkanDevice, VulkanFilter, VulkanFormat, VulkanSampler, VulkanSamplerAddressMode,
    VulkanSamplerMipmapMode, VulkanShaderStageFlag, compile_shader,
};

compile_shader! {
    const FRAGMENT_SHADER = "tone-map.slang",
    frag_main
}

impl ToneMapNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render::frame_graph::nodes) fn create_fixed_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        _: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Create post processing descriptor set layout
        fixed_render_objects.add_descriptor_set_layout(
            &[VulkanDescriptorSetLayoutBinding::new(
                0,
                VulkanDescriptorType::CombinedImageSampler,
                1,
                VulkanShaderStageFlag::Fragment,
            )],
            3,
            FixedRenderObjects::POST_PROCESS_DESCRIPTOR_SET_LAYOUT,
            device,
        )?;

        // Create post processing sampler
        fixed_render_objects.add_sampler(
            create_sampler(device)?,
            FixedRenderObjects::LINEAR_CLAMP_SAMPLER,
        );

        // Create the pipeline
        let shader = Shader::new(&FRAGMENT_SHADER, device)?;
        fixed_render_objects.add_pipeline(
            Pipeline::new_post_process(
                fixed_render_objects.fullscreen_quad(),
                &shader,
                std::mem::size_of::<PushConstants>(),
                None,
                &[SDR_FORMAT],
                &[fixed_render_objects
                    .descriptor_set_layout(FixedRenderObjects::POST_PROCESS_DESCRIPTOR_SET_LAYOUT)],
                device,
            )?,
            FixedRenderObjects::TONE_MAP_PIPELINE,
        );

        Ok(())
    }
}

fn create_sampler(device: &VulkanDevice) -> Result<VulkanSampler> {
    device
        .create_sampler(
            0,
            VulkanFilter::Linear,
            VulkanFilter::Linear,
            VulkanSamplerMipmapMode::Linear,
            VulkanSamplerAddressMode::ClampToEdge,
            VulkanSamplerAddressMode::ClampToEdge,
            VulkanSamplerAddressMode::ClampToEdge,
            0.0,
            false,
            1.0,
            false,
            VulkanCompareOp::Always,
            0.0,
            1.0,
            VulkanBorderColor::FloatTransparentBlack,
            false,
        )
        .map_err(Error::new_inner)
}
