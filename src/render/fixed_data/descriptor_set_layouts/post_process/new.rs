use crate::{
    Result,
    render::{DescriptorSetLayouts, GraphicsDevice, PostProcessDsl},
};

impl PostProcessDsl {
    /// Create a new set of [`PostProcessDsl`] resources
    pub(in crate::render::fixed_data::descriptor_set_layouts) fn new(
        descriptor_set_layouts: &mut DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<()> {
        descriptor_set_layouts.create::<PostProcessDsl>("Post Process", device)
    }
}
