use crate::{
    Result,
    render::{DescriptorSetLayouts, DirectionalLightDsl, GraphicsDevice},
};

impl DirectionalLightDsl {
    /// Create a new set of [`DirectionalLightDsl`] resources
    pub(in crate::render::fixed_data::descriptor_set_layouts) fn new(
        descriptor_set_layouts: &mut DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<()> {
        descriptor_set_layouts.create::<DirectionalLightDsl>("Directional Light", device)
    }
}
