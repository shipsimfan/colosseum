use crate::{
    Result,
    render::{DescriptorSetLayouts, GraphicsDevice, LightsDsl},
};

impl LightsDsl {
    /// Create a new set of [`LightsDsl`] resources
    pub(in crate::render::fixed_data::descriptor_set_layouts) fn new(
        descriptor_set_layouts: &mut DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<()> {
        descriptor_set_layouts.create::<LightsDsl>("All Lights", device)
    }
}
