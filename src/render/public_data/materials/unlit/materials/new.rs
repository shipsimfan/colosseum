use crate::{
    Result,
    render::{
        CameraDsl, DescriptorSetLayouts, GraphicsDevice, RenderableDsl, UnlitMaterial, UnlitShader,
        public_data::materials::{UnlitMaterialPushConstants, UnlitMaterials},
    },
};
use alexandria::{
    SlotMap,
    gpu::{VulkanPushConstantRange, VulkanShaderStageFlag, compile_shader},
    math::Color4f,
};
use colosseum_core::new_error;

compile_shader! {
    /// The default shader for unlit opaque materials
    const DEFAULT_SHADER = "default.slang",
    vert_main,
    frag_main
}

impl UnlitMaterials {
    /// Create a new set of [`UnlitMaterials`]
    pub fn new(
        descriptor_set_layouts: &DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<UnlitMaterials> {
        // Create pipeline layout
        let mut pipeline_layout = device
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
            .map_err(|error| new_error!("unable to create unlit pipeline layout - {}", error))?;

        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut pipeline_layout, c"Unlit Pipeline Layout")
            .map_err(|error| {
                new_error!("unable to set name for unlit pipeline layout - {}", error)
            })?;

        // Create the default unlit shader
        let default_shader = UnlitShader::new(&DEFAULT_SHADER, &pipeline_layout, device)?;

        // Create the default unlit material
        let mut materials = SlotMap::new();
        let default_material =
            materials.insert(UnlitMaterial::new(Color4f::WHITE, default_shader.clone()));

        Ok(UnlitMaterials {
            materials,
            default_material,
            default_shader,
            pipeline_layout,
        })
    }
}
