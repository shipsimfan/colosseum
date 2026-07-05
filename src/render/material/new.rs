use crate::{
    Result,
    render::{Material, RenderMaterial, Shader},
};
use alexandria::gpu::{VulkanDevice, VulkanFormat, VulkanPipelineLayout};
use std::sync::Arc;

impl Material {
    /// Create a new [`Material`] and [`RenderMaterial`]
    pub(crate) fn new(
        shader: &Arc<Shader>,
        pipeline_layout: &VulkanPipelineLayout,
        swapchain_format: VulkanFormat,
        device: &VulkanDevice,
    ) -> Result<(Material, RenderMaterial)> {
        let render_material =
            RenderMaterial::new(shader, pipeline_layout, swapchain_format, device)?;

        Ok((Material {}, render_material))
    }
}
