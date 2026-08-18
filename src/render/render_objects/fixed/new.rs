use crate::{
    Error, Result,
    render::{FixedRenderObjects, FrameGraphNode, RenderMaterial, Shader},
};
use alexandria::gpu::{
    VulkanDescriptorSetLayoutBinding, VulkanDescriptorType, VulkanDevice, VulkanFormat,
    VulkanPushConstantRange, VulkanShaderStageFlag, compile_shader,
};
use std::sync::Arc;

compile_shader! {
    /// The vertex shader code for the fullscreen quad shader
    const FULLSCREEN_QUAD_SHADER = "fullscreen-quad.slang",
    vert_main
}

impl FixedRenderObjects {
    /// Create a new set of [`FixedRenderObjects`]
    pub fn new(
        swapchain_format: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<Arc<FixedRenderObjects>> {
        // Create the descriptor set layout for the camera data
        let camera_data_layout = device
            .create_descriptor_set_layout(
                0,
                &[VulkanDescriptorSetLayoutBinding::new(
                    0,
                    VulkanDescriptorType::UniformBuffer,
                    1,
                    VulkanShaderStageFlag::Vertex,
                )],
            )
            .map_err(Error::new_inner)?;

        // Create the pipeline layout for unlit opaque rendering
        let unlit_forward_pipeline_layout = device
            .create_pipeline_layout(
                0,
                &[&camera_data_layout],
                &[VulkanPushConstantRange::new(
                    VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
                    0,
                    RenderMaterial::PUSH_CONSTANT_SIZE as _,
                )],
            )
            .map_err(Error::new_inner)?;

        // Create the persistent objects that are used by nodes
        let mut pipelines = Vec::new();
        let fullscreen_quad = Shader::new(&FULLSCREEN_QUAD_SHADER, device)?;

        FrameGraphNode::create_objects(&mut pipelines, &fullscreen_quad, swapchain_format, device)?;

        Ok(Arc::new(FixedRenderObjects {
            camera_data_layout,
            unlit_forward_pipeline_layout,
            pipelines,
        }))
    }
}
