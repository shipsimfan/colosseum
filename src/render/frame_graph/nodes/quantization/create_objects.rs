use crate::{
    Error, Result,
    render::{
        FixedRenderObjects, Pipeline, RenderData, Shader,
        frame_graph::{QuantizationNode, nodes::quantization::PushConstants},
    },
};
use alexandria::gpu::{
    VulkanDescriptorPool, VulkanDescriptorSet, VulkanDevice, VulkanFormat, compile_shader,
};

compile_shader! {
    const FRAGMENT_SHADER = "quantization.slang",
    frag_main
}

impl QuantizationNode {
    /// Create the persistent objects that are used by this node
    pub(in crate::render) fn create_objects(
        fixed_render_objects: &mut FixedRenderObjects,
        swapchain_format: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<()> {
        // Create the pipeline
        let shader = Shader::new(&FRAGMENT_SHADER, device)?;
        fixed_render_objects.add_pipeline(
            Pipeline::new_post_process(
                fixed_render_objects.fullscreen_quad(),
                &shader,
                std::mem::size_of::<PushConstants>(),
                None,
                &[swapchain_format],
                &[fixed_render_objects
                    .descriptor_set_layout(FixedRenderObjects::POST_PROCESS_DESCRIPTOR_SET_LAYOUT)],
                device,
            )?,
            FixedRenderObjects::QUANTIZATION_PIPELINE,
        );

        Ok(())
    }

    /// Create needed per-frame descriptor sets for this node
    pub(in crate::render) fn create_descriptor_sets(
        fixed_render_objects: &FixedRenderObjects,
        descriptor_pool: &mut VulkanDescriptorPool,
        descriptor_sets: &mut Vec<VulkanDescriptorSet>,
    ) -> Result<()> {
        assert_eq!(
            RenderData::QUANTIZATION_DESCRIPTOR_SET,
            descriptor_sets.len()
        );

        let descriptor_set = descriptor_pool
            .allocate_descriptor_set(
                fixed_render_objects
                    .descriptor_set_layout(FixedRenderObjects::POST_PROCESS_DESCRIPTOR_SET_LAYOUT),
            )
            .map_err(Error::new_inner)?;
        descriptor_sets.push(descriptor_set);

        Ok(())
    }
}
