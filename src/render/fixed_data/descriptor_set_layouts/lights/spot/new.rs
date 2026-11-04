use crate::{
    Result,
    render::{DescriptorSetLayouts, GraphicsDevice, SpotLightDsl},
};

impl SpotLightDsl {
    /// Create a new set of [`SpotLightDsl`] resources
    pub(in crate::render::fixed_data::descriptor_set_layouts) fn new(
        descriptor_set_layouts: &mut DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<()> {
        descriptor_set_layouts.create::<SpotLightDsl>("Spot Light", device)
    }
}
