use crate::{
    Result,
    render::{DescriptorSetLayouts, GraphicsDevice, PostProcess},
};

impl PostProcess {
    /// Create a new set of [`PostProcess`] resources
    pub(in crate::render::fixed_data) fn new(
        descriptor_set_layouts: &mut DescriptorSetLayouts,
        device: &GraphicsDevice,
    ) -> Result<()> {
        descriptor_set_layouts.create::<PostProcess>("Post Process Descriptor Set Layout", device)
    }
}
