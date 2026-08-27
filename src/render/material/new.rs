use crate::{
    Result,
    render::{Material, RenderMaterial, Shader},
};
use alexandria::{
    gpu::{VulkanDevice, VulkanPipelineLayout},
    math::Color4f,
};
use std::sync::Arc;

impl Material {
    /// Create a new [`Material`] and [`RenderMaterial`]
    pub(crate) fn new(
        shader: &Arc<Shader>,
        pipeline_layout: &VulkanPipelineLayout,
        device: &VulkanDevice,
    ) -> Result<(Material, RenderMaterial)> {
        let render_material = RenderMaterial::new(shader, pipeline_layout, device)?;

        Ok((
            Material {
                color: Color4f::WHITE,
            },
            render_material,
        ))
    }
}
