use crate::{
    Result,
    render::{
        CameraDsl, DescriptorSetLayouts, GraphicsDevice, Materials, RenderableDsl, UnlitMaterial,
        UnlitShader, public_data::materials::UnlitMaterialPushConstants,
    },
};
use alexandria::{
    SlotMap,
    gpu::{VulkanPushConstantRange, VulkanShaderStageFlag, compile_shader},
    math::Color4f,
};
use colosseum_core::new_error;
use std::sync::Arc;

compile_shader! {
    /// The default shader for unlit opaque materials
    const UNLIT_SHADER = "unlit/default.slang",
    vert_main,
    frag_main
}

impl Materials {
    /// Create a new set of [`Materials`]
    pub(in crate::render::public_data) fn new(
        descriptor_set_layouts: &DescriptorSetLayouts,
        device: &Arc<GraphicsDevice>,
    ) -> Result<Materials> {
        let mut unlit_pipeline_layout = device
            .create_pipeline_layout(
                0,
                &[
                    descriptor_set_layouts.get::<CameraDsl>(),
                    descriptor_set_layouts.get::<RenderableDsl>(),
                ],
                &[VulkanPushConstantRange::new(
                    VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
                    0,
                    std::mem::size_of::<UnlitMaterialPushConstants>() as _,
                )],
            )
            .map_err(|error| new_error!("unable to create default unlit shader - {}", error))?;

        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut unlit_pipeline_layout, c"Unlit Pipeline Layout")
            .map_err(|error| {
                new_error!("unable to set name for unlit pipeline layout - {}", error)
            })?;

        let default_unlit_shader = UnlitShader::new(&UNLIT_SHADER, &unlit_pipeline_layout, device)?;

        let mut unlit_materials = SlotMap::new();
        let default_unlit_material = unlit_materials.insert(UnlitMaterial::new(
            Color4f::WHITE,
            default_unlit_shader.clone(),
        ));

        Ok(Materials {
            unlit_materials,
            default_unlit_material,
            default_unlit_shader,
            unlit_pipeline_layout,

            device: device.clone(),
        })
    }
}
