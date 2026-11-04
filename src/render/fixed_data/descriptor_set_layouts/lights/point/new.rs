use crate::{
    Result,
    render::{DescriptorSetLayouts, GraphicsDevice, PointLightDsl},
};

impl PointLightDsl {
    /// Create a new set of [`PointLightDsl`] resources
    pub(in crate::render::fixed_data::descriptor_set_layouts) fn new(
        descriptor_set_layouts: &mut DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<()> {
        descriptor_set_layouts.create::<PointLightDsl>("Point Light", device)
    }
}
