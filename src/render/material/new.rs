use crate::{
    Result,
    render::{Material, MaterialKind, RenderMaterial, Shader},
};
use alexandria::{
    gpu::{VulkanDevice, VulkanPipelineLayout},
    math::Color4f,
};
use std::sync::Arc;

impl Material {
    /// Create a new [`Material`] and [`RenderMaterial`]
    pub(crate) fn new(
        kind: MaterialKind,
        shader: &Arc<Shader>,
        pipeline_layout: &VulkanPipelineLayout,
        device: &VulkanDevice,
    ) -> Result<(Material, RenderMaterial)> {
        let render_material = RenderMaterial::new(kind, shader, pipeline_layout, device)?;

        Ok((
            Material {
                color: Color4f::WHITE,
                specular_strength: 0.5,
                shininess: 32.0,
            },
            render_material,
        ))
    }
}
