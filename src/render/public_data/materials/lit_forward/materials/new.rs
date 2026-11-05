use crate::{
    Result,
    render::{
        CameraDsl, DescriptorSetLayouts, GraphicsDevice, LightsDsl, LitForwardMaterial,
        LitForwardShader, RenderableDsl,
        public_data::materials::{LitForwardMaterialPushConstants, LitForwardMaterials},
    },
};
use alexandria::{
    SlotMap,
    gpu::{VulkanPushConstantRange, VulkanShaderStageFlag, compile_shader},
    math::Color4f,
};
use colosseum_core::new_error;

compile_shader! {
    /// The default shader for lit forward opaque materials
    const DEFAULT_SHADER = "default.slang",
    vert_main,
    frag_main
}

impl LitForwardMaterials {
    /// Create a new set of [`LitForwardMaterials`]
    pub fn new(
        descriptor_set_layouts: &DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<LitForwardMaterials> {
        // Create the forward render pipeline layout
        let mut pipeline_layout = device
            .create_pipeline_layout(
                0,
                &[
                    descriptor_set_layouts.get::<CameraDsl>(),
                    descriptor_set_layouts.get::<RenderableDsl>(),
                    descriptor_set_layouts.get::<LightsDsl>(),
                ],
                &[VulkanPushConstantRange::new(
                    VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
                    0,
                    std::mem::size_of::<LitForwardMaterialPushConstants>() as _,
                )],
            )
            .map_err(|error| {
                new_error!("unable to create lit forward pipeline layout - {}", error)
            })?;

        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut pipeline_layout, c"Lit Forward Pipeline Layout")
            .map_err(|error| {
                new_error!(
                    "unable to set name for lit forward pipeline layout - {}",
                    error
                )
            })?;

        // Create the default lit forward shader
        let default_shader = LitForwardShader::new(&DEFAULT_SHADER, &pipeline_layout, device)?;

        // Create the default lit forward material
        let mut materials = SlotMap::new();
        let default_material = materials.insert(LitForwardMaterial::new(
            Color4f::WHITE,
            0.5,
            32.0,
            default_shader.clone(),
        ));

        Ok(LitForwardMaterials {
            materials,
            default_material,
            default_shader,
            pipeline_layout,
        })
    }
}
